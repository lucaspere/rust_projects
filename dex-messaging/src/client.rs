use crate::config::MessagingConfig;
use crate::error::{
    IggySnafu, NatsConnectSnafu, NatsPublishSnafu, NatsSnafu, NatsSubscribeSnafu,
    SerializationSnafu,
};
use crate::model::{Event, PersistentEvent, RealtimeEvent};
use crate::Result;
use futures::StreamExt;
use iggy::client::{Client, MessageClient, UserClient}; // Importe o trait Client
use iggy::client::{StreamClient, TopicClient};
use iggy::clients::client::IggyClient;
use iggy::compression::compression_algorithm::CompressionAlgorithm;
use iggy::consumer::Consumer;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::poll_messages::{PollMessages, PollingStrategy};
use iggy::messages::send_messages::{Message, Partitioning, SendMessages};
use iggy::streams::create_stream::CreateStream;
use iggy::tcp::client::TcpClient;
use iggy::tcp::config::{TcpClientConfig, TcpClientReconnectionConfig};
use iggy::topics::create_topic::CreateTopic;
use iggy::utils::duration::IggyDuration;
use iggy::utils::expiry::IggyExpiry;
use iggy::utils::topic_size::MaxTopicSize;
use snafu::ResultExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

#[derive(Clone)]
pub struct MessagingClient {
    client: Arc<IggyClient>,
    nats_client: Arc<async_nats::Client>,
}

impl MessagingClient {
    pub async fn new(config: MessagingConfig) -> Result<Self> {
        let transport_config = Arc::new(TcpClientConfig {
            server_address: config.iggy_server_address.clone(),
            reconnection: TcpClientReconnectionConfig {
                enabled: true,
                interval: config.reconnect_interval_ms.into(),
                max_retries: config.max_reconnect_retries,
                reestablish_after: IggyDuration::from(config.connect_timeout_ms),
            },
            heartbeat_interval: config.reconnect_interval_ms.into(),
            ..Default::default()
        });
        let client = Box::new(TcpClient::create(transport_config).context(IggySnafu)?);
        let client = IggyClient::create(client, None, None); // Use IggyClient::create
        client.connect().await.context(IggySnafu)?;
        if let Some(credentials) = config.login_credentials {
            client
                .login_user(credentials.username.as_str(), credentials.password.as_str())
                .await
                .context(IggySnafu)?;
        }

        let nats_client = async_nats::connect(&config.nats_server_address)
            .await
            .context(NatsConnectSnafu)?;
        info!("Successfully connected to NATS server.");

        Ok(Self {
            client: Arc::new(client),
            nats_client: Arc::new(nats_client),
        })
    }

    pub async fn publish<E: Event>(&self, event: &E) -> Result<()> {
        let payload = serde_json::to_vec(event).context(SerializationSnafu)?;
        let message = Message::new(None, payload.into(), None);

        let mut command = SendMessages {
            stream_id: E::stream_id(),
            topic_id: E::topic_id(),
            partitioning: Partitioning::default(),
            messages: vec![message],
        };

        self.client
            .send_messages(
                &command.stream_id,
                &command.topic_id,
                &Partitioning::balanced(),
                &mut command.messages,
            )
            .await
            .context(IggySnafu)?;
        Ok(())
    }

    pub async fn publish_batch<E: PersistentEvent>(&self, events: &[E]) -> Result<()> {
        // Publica um lote de eventos de uma vez.
        if events.is_empty() {
            return Ok(());
        }

        let messages: Vec<Message> = events
            .iter()
            .map(|event| {
                let payload = event.encode_to_vec();
                Message::new(None, payload.into(), None)
            })
            .collect();

        let mut command = SendMessages {
            stream_id: E::stream_id(),
            topic_id: E::topic_id(),
            partitioning: Partitioning::default(),
            messages,
        };

        self.client
            .send_messages(
                &command.stream_id,
                &command.topic_id,
                &Partitioning::balanced(),
                &mut command.messages,
            )
            .await
            .context(IggySnafu)?;

        Ok(())
    }

    pub async fn consume<E: PersistentEvent, F>(
        &self,
        consumer_name: &str,
        mut handler: F,
    ) -> Result<()>
    where
        F: FnMut(E) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>,
    {
        // Um consumidor é identificado pelo seu grupo e nome.
        // Usar o nome do evento ajuda a criar IDs únicos.
        let consumer_id = E::event_name();
        println!(
            "Starting consumer '{}' for event '{}' on Stream({}), Topic({})",
            consumer_name,
            E::event_name(),
            E::stream_id(),
            E::topic_id()
        );

        let command = PollMessages {
            consumer: Consumer::new(Identifier::named(consumer_id).context(IggySnafu)?),
            stream_id: E::stream_id(),
            topic_id: E::topic_id(),
            partition_id: Some(1), // Para simplificar; pode ser mais complexo
            strategy: PollingStrategy::next(),
            count: 10, // Pega até 10 mensagens por vez
            auto_commit: true,
        };

        loop {
            let polled = self
                .client
                .poll_messages(
                    &command.stream_id,
                    &command.topic_id,
                    command.partition_id,
                    &command.consumer,
                    &command.strategy,
                    command.count,
                    command.auto_commit,
                )
                .await;
            match polled {
                Ok(messages) if !messages.messages.is_empty() => {
                    for msg in messages.messages {
                        match E::decode(&msg.payload[..]) {
                            Ok(event) => {
                                if let Err(e) = handler(event) {
                                    eprintln!("Error processing event: {:?}", e);
                                }
                            }
                            Err(e) => eprintln!("Failed to deserialize event: {:?}", e),
                        }
                    }
                }
                Ok(_) => {
                    // Nenhuma mensagem, espera um pouco para não sobrecarregar a CPU
                    sleep(Duration::from_millis(100)).await;
                }
                Err(e) => {
                    println!("Failed to poll messages: {:?}. Retrying...", e);
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    pub async fn ensure_stream_and_topic<E: PersistentEvent>(&self) -> Result<()> {
        let stream_id = E::stream_id();
        let topic_id = E::topic_id();

        // Garante que o Stream existe
        let create_stream = CreateStream {
            stream_id: stream_id.get_u32_value().ok(),
            name: format!("stream_{}", stream_id), // Nomeie de forma consistente
        };
        match self
            .client
            .create_stream(&create_stream.name, stream_id.get_u32_value().ok())
            .await
        {
            Ok(_) => println!("Stream {} created.", stream_id),
            Err(IggyError::StreamIdAlreadyExists(..) | IggyError::StreamNameAlreadyExists(..)) => {}
            Err(e) => return Err(e).context(IggySnafu),
        }

        // Garante que o Topic existe
        let create_topic = CreateTopic {
            stream_id: stream_id.clone(),
            topic_id: topic_id.get_u32_value().ok(),
            partitions_count: 1, // Comece com 1, aumente se precisar
            name: format!("topic_{}", topic_id),
            message_expiry: IggyExpiry::NeverExpire,
            max_topic_size: MaxTopicSize::default(),
            replication_factor: Some(1),
            compression_algorithm: CompressionAlgorithm::None,
        };
        match self
            .client
            .create_topic(
                &create_topic.stream_id,
                &create_topic.name,
                create_topic.partitions_count,
                create_topic.compression_algorithm,
                create_topic.replication_factor,
                create_topic.topic_id,
                create_topic.message_expiry,
                create_topic.max_topic_size,
            )
            .await
        {
            Ok(_) => println!("Topic {}/{} created.", stream_id, topic_id),
            Err(IggyError::TopicIdAlreadyExists(..) | IggyError::TopicNameAlreadyExists(..)) => {
                /* Já existe, tudo bem */
            }
            Err(e) => return Err(e).context(IggySnafu),
        }

        Ok(())
    }

    /// Publica um evento em tempo real no NATS.
    /// Extremamente rápido e leve ("fire and forget").
    pub async fn publish_realtime<E: RealtimeEvent>(&self, event: &E) -> Result<()> {
        let subject = event.subject();
        let payload = event.encode_to_vec();

        debug!(subject = %subject, "Publishing realtime event");
        self.nats_client
            .publish(subject, payload.into())
            .await
            .context(NatsPublishSnafu)?;

        Ok(())
    }

    /// Inscreve-se em um assunto NATS (pode usar wildcards, ex: "prices.*")
    /// e processa as mensagens recebidas com um handler.
    /// Esta função entra em um loop infinito para escutar continuamente.
    pub async fn subscribe_realtime<E, F>(&self, subject: &str, mut handler: F) -> Result<()>
    where
        E: RealtimeEvent, // O tipo de evento que esperamos deserializar
        F: FnMut(E) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>,
    {
        info!(subject = %subject, "Subscribing to realtime subject");
        let mut sub = self
            .nats_client
            .subscribe(subject.to_string())
            .await
            .context(NatsSubscribeSnafu)?;

        while let Some(msg) = sub.next().await {
            match E::decode(&msg.payload[..]) {
                Ok(event) => {
                    if let Err(e) = handler(event) {
                        error!(subject = %msg.subject, error = %e, "Error processing realtime event");
                    }
                }
                Err(e) => {
                    warn!(subject = %msg.subject, error = %e, "Failed to deserialize realtime event payload");
                }
            }
        }

        Ok(())
    }
}
