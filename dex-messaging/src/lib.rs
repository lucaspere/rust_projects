//! # Crate de Mensageria Compartilhada para a DEX
//!
//! Esta crate fornece uma abstração sobre o Iggy para permitir
//! que diferentes serviços publiquem e consumam eventos de forma
//! consistente e genérica.
//!
//! ## Exemplo de Uso (em um serviço consumidor/produtor)
//!
//! ```rust,ignore
//! use shared_messaging::{MessagingClient, MessagingConfig, Event};
//! use shared_messaging::model::{SwapCompleted, UserScored}; // Importe seus eventos
//!
//! #[tokio::main]
//! async fn main() {
//!     // 1. Carregue a configuração (ex: de .env para desenvolvimento)
//!     dotenv::dotenv().ok();
//!     let config = MessagingConfig::from_env().expect("Failed to load config");
//!
//!     // 2. Crie o cliente de mensageria
//!     let client = MessagingClient::new(config).await.expect("Failed to connect");
//!     let producer_client = client.clone();
//!     
//!     // 3. Inicie um consumidor em uma task separada
//!     tokio::spawn(async move {
//!         client.consume("scoring_service", |event: SwapCompleted| {
//!             println!("Evento SwapCompleted recebido! Processando TX: {}", event.transaction_id);
//!             // ... lógica do job de score ...
//!             Ok(())
//!         }).await.unwrap();
//!     });
//!
//!     // 4. Publique um evento de outro lugar na aplicação
//!     let swap_event = SwapCompleted {
//!         transaction_id: "tx-12345".to_string(),
//!         user_id: 100,
//!         token_in: "USDC".to_string(),
//!         token_out: "RUST_COIN".to_string(),
//!         amount_in: 1000.0,
//!     };
//!
//!     producer_client.publish(&swap_event).await.expect("Failed to publish event");
//!     println!("Evento SwapCompleted publicado!");
//! }
//! ```
pub mod client;
pub mod config;
pub mod error;
pub mod model;

pub use client::MessagingClient;
pub use config::MessagingConfig;
pub use error::{MessagingError, Result};
pub use model::Event;
