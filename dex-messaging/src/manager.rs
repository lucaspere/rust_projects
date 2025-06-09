use crate::prelude::*;
use crate::providers::StreamProvider;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::{info, warn};

struct ManagedConnection {
    ref_count: usize,
    task_handle: JoinHandle<()>,
}

#[derive(Default)]
struct ConnectionManager {
    active_streams: HashMap<String, ManagedConnection>,
}

#[derive(Clone)]
pub struct SubscriptionService {
    manager: Arc<Mutex<ConnectionManager>>,
    publisher: Arc<dyn RealtimePublisher>,
    provider: Arc<dyn StreamProvider>,
}

impl SubscriptionService {
    pub fn new(publisher: Arc<dyn RealtimePublisher>, provider: Arc<dyn StreamProvider>) -> Self {
        Self {
            manager: Arc::new(Mutex::new(ConnectionManager::default())),
            publisher,
            provider,
        }
    }

    /// Um consumidor se inscreve para receber dados de um `stream_id`.
    /// Retorna um `Subscription` que deve ser mantido pelo consumidor.
    /// Quando o `Subscription` for descartado (`drop`), ele automaticamente
    /// chamará `unsubscribe`.
    pub async fn subscribe(&self, stream_id: String) -> Subscription {
        let mut manager = self.manager.lock().await;

        // Caso 1: O stream já está ativo. Apenas incrementamos o contador.
        if let Some(conn) = manager.active_streams.get_mut(&stream_id) {
            conn.ref_count += 1;
            info!(stream_id = %stream_id, ref_count = conn.ref_count, "Reusing existing connection");
            return Subscription::new(stream_id, self.clone());
        }

        // Caso 2: O stream não está ativo. Precisamos iniciar uma nova conexão.
        info!(stream_id = %stream_id, "Establishing new provider connection");

        // Clonamos os Arcs para movê-los para a nova task
        let provider = self.provider.clone();
        let publisher = self.publisher.clone();
        let task_stream_id = stream_id.clone();

        let task_handle = tokio::spawn(async move {
            if let Err(e) = provider.start_stream(task_stream_id, publisher).await {
                warn!("Stream provider task exited with error: {}", e);
            }
        });

        let new_conn = ManagedConnection {
            ref_count: 1,
            task_handle,
        };

        manager.active_streams.insert(stream_id.clone(), new_conn);

        Subscription::new(stream_id, self.clone())
    }

    async fn unsubscribe(&self, stream_id: &str) {
        let mut manager = self.manager.lock().await;

        if let Some(conn) = manager.active_streams.get_mut(stream_id) {
            conn.ref_count -= 1;
            info!(stream_id = %stream_id, ref_count = conn.ref_count, "Consumer unsubscribed");

            if conn.ref_count == 0 {
                info!(stream_id = %stream_id, "Last consumer left. Terminating provider connection.");
                conn.task_handle.abort();
                manager.active_streams.remove(stream_id);
            }
        } else {
            warn!(stream_id = %stream_id, "Attempted to unsubscribe from a non-existent stream");
        }
    }
}

pub struct Subscription {
    stream_id: String,
    service: SubscriptionService,
}

impl Subscription {
    fn new(stream_id: String, service: SubscriptionService) -> Self {
        Self { stream_id, service }
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        let service = self.service.clone();
        let stream_id = self.stream_id.clone();
        tokio::spawn(async move {
            service.unsubscribe(&stream_id).await;
        });
    }
}
