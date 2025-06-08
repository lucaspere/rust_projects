use crate::config::IggyConfig;
use crate::error::{IggyClientSnafu, IggySnafu};
use crate::model::PersistentEvent;
use crate::traits::Persistent;
use crate::DexMessagingResult;
use async_trait::async_trait;
use iggy::{
    client::{Client, MessageClient, StreamClient, TopicClient, UserClient},
    clients::client::IggyClient,
    compression::compression_algorithm::CompressionAlgorithm,
    consumer::Consumer,
    error::IggyError,
    identifier::Identifier,
    messages::{
        poll_messages::{PollMessages, PollingStrategy},
        send_messages::{Message, Partitioning, SendMessages},
    },
    streams::create_stream::CreateStream,
    tcp::client::TcpClient,
    tcp::config::{TcpClientConfig, TcpClientReconnectionConfig},
    topics::create_topic::CreateTopic,
    utils::{duration::IggyDuration, expiry::IggyExpiry, topic_size::MaxTopicSize},
};
use snafu::prelude::*;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct IggyMessagingClient {
    client: IggyClient,
}

impl IggyMessagingClient {
    pub async fn new(config: IggyConfig) -> DexMessagingResult<Self> {
        let transport_config = Arc::new(TcpClientConfig {
            server_address: config.server_address.clone(),
            reconnection: TcpClientReconnectionConfig {
                enabled: true,
                interval: config.reconnect_interval_ms.into(),
                max_retries: config.max_reconnect_retries,
                reestablish_after: IggyDuration::from(config.connect_timeout_ms),
            },
            heartbeat_interval: config.reconnect_interval_ms.into(),
            ..Default::default()
        });
        let client = Box::new(
            TcpClient::create(transport_config)
                .context(IggyClientSnafu)
                .context(IggySnafu)?,
        );
        let client = IggyClient::create(client, None, None);
        client
            .connect()
            .await
            .context(IggyClientSnafu)
            .context(IggySnafu)?;

        if let Some(credentials) = config.login_credentials {
            client
                .login_user(credentials.username.as_str(), credentials.password.as_str())
                .await
                .context(IggyClientSnafu)
                .context(IggySnafu)?;
        }

        Ok(Self { client })
    }
}

#[async_trait]
impl Persistent for IggyMessagingClient {
    async fn publish<E: PersistentEvent>(&self, event: &E) -> DexMessagingResult<()> {
        let messages = vec![Message::new(None, event.encode_to_vec().into(), None)];

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
            .context(IggyClientSnafu)
            .context(IggySnafu)?;

        Ok(())
    }

    async fn publish_batch<E: PersistentEvent>(&self, events: &[E]) -> DexMessagingResult<()> {
        self.internal_publish_batch(events).await
    }

    async fn consume<E, F>(&self, subject: &str, mut handler: F) -> DexMessagingResult<()>
    where
        E: PersistentEvent,
        F: FnMut(E) -> DexMessagingResult<()> + Send,
    {
        // Um consumidor é identificado pelo seu grupo e nome.
        // Usar o nome do evento ajuda a criar IDs únicos.
        let consumer_id = E::event_name();
        println!(
            "Starting consumer '{}' for event '{}' on Stream({}), Topic({})",
            consumer_id,
            E::event_name(),
            E::stream_id(),
            E::topic_id()
        );

        let command = PollMessages {
            consumer: Consumer::group(
                Identifier::named(consumer_id)
                    .context(IggyClientSnafu)
                    .context(IggySnafu)?,
            ),
            stream_id: E::stream_id(),
            topic_id: E::topic_id(),
            partition_id: None,
            strategy: PollingStrategy::next(),
            count: 10, // Pega até 10 mensagens por vez
            auto_commit: false,
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
                Ok(messages_result) if !messages_result.messages.is_empty() => {
                    let current_partition_id = messages_result.partition_id;
                    let mut last_processed_offset: u64 = 0;
                    for msg in messages_result.messages {
                        match E::decode(&msg.payload.as_ref()[..]) {
                            // Use as_ref() para &[u8]
                            Ok(event) => {
                                if let Err(e) = handler(event) {
                                    eprintln!(
                                        "Error processing event at offset {}: {:?}",
                                        msg.offset, e
                                    );
                                    // Não atualiza o offset se houve erro, permitindo re-entrega.
                                } else {
                                    last_processed_offset = msg.offset; // Armazena o offset da última mensagem processada com sucesso
                                    println!(
                                        "Successfully processed event at offset {}",
                                        msg.offset
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!(
                                    "Failed to deserialize event at offset {}: {:?}",
                                    msg.offset, e
                                );
                                // Para mensagens mal-formadas, você pode querer pular e dar ACK para não travar a fila.
                                // Se você der ACK aqui, elas não serão re-entregues.
                                // Para este exemplo, vamos dar ACK mesmo se mal-formado para continuar.
                                last_processed_offset = msg.offset;
                                println!(
                                    "Acknowledged malformed message with offset: {}",
                                    msg.offset
                                );
                            }
                        }
                    }

                    if last_processed_offset > 0 {
                        self.client
                            .store_consumer_offset(
                                &command.consumer,
                                &command.stream_id,
                                &command.topic_id,
                                Some(current_partition_id), // Usar o partition_id do PolledMessages
                                last_processed_offset,
                            )
                            .await
                            .context(IggyClientSnafu)
                            .context(IggySnafu)?;
                        println!(
                            "Stored consumer offset for partition {} at offset {}",
                            current_partition_id, last_processed_offset
                        );
                    }
                }
                Ok(_) => {
                    // Nenhuma mensagem, espera um pouco para não sobrecarregar a CPU
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                Err(e) => {
                    println!("Failed to poll messages: {:?}. Retrying...", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}

impl IggyMessagingClient {
    async fn internal_publish_batch<E: PersistentEvent>(
        &self,
        events: &[E],
    ) -> DexMessagingResult<()> {
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
            .context(IggyClientSnafu)
            .context(IggySnafu)?;

        Ok(())
    }

    pub async fn ensure_stream_and_topic<E: PersistentEvent>(&self) -> DexMessagingResult<()> {
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
            Err(e) => return Err(e).context(IggyClientSnafu).context(IggySnafu),
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
            Err(e) => return Err(e).context(IggyClientSnafu).context(IggySnafu),
        }

        Ok(())
    }
}
