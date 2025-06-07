use bullpen_dex_messaging::{model::SwapCompleted, MessagingClient, MessagingConfig};
use dotenv::dotenv;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

static PROCESSED_COUNT: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("🚀 Starting DEX Messaging Consumer...");

    let iggy_client = MessagingClient::new(MessagingConfig::from_env()?).await?;
    info!("✅ Iggy client initialized successfully.");

    // Garantir que stream e topic existem
    iggy_client
        .ensure_stream_and_topic::<SwapCompleted>()
        .await?;
    info!("✅ Stream and Topic ensured for SwapCompleted events");

    // Iniciar publicação de eventos de teste em background
    let client_for_publisher = iggy_client.clone();
    tokio::spawn(async move {
        if let Err(e) = event_publisher(client_for_publisher).await {
            error!("Event publisher failed: {}", e);
        }
    });

    // Contador de eventos processados
    let processed_count = Arc::new(AtomicU64::new(0));
    let count_clone = processed_count.clone();

    // Task para mostrar estatísticas
    tokio::spawn(async move {
        let mut last_count = 0;
        loop {
            sleep(Duration::from_secs(30)).await;
            let current_count = count_clone.load(Ordering::Relaxed);
            let rate = current_count - last_count;
            info!(
                "📊 Events processed: {} (Rate: {}/30s)",
                current_count, rate
            );
            last_count = current_count;
        }
    });

    // Handler realista para processar eventos de swap
    let handler =
        move |event: SwapCompleted| -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let count = processed_count.fetch_add(1, Ordering::Relaxed) + 1;

            info!(
                "🔄 Processing SwapCompleted #{}: User {} swapped {:.2} {} → {} (TX: {})",
                count,
                event.user_id,
                event.amount_in,
                event.token_in,
                event.token_out,
                &event.transaction_id[..8] // Mostrar apenas primeiros 8 chars do hash
            );

            // Simular processamento real com diferentes cenários
            simulate_business_logic(&event)?;

            info!(
                "✅ SwapCompleted #{} processed successfully for user {}",
                count, event.user_id
            );
            Ok(())
        };

    info!("🎯 Starting event consumption for SwapCompleted events...");

    // Inicia o consumo de eventos
    iggy_client
        .consume("swap_completed_consumer", handler)
        .await?;

    Ok(())
}

/// Simula lógica de negócio realista para processamento de swaps
fn simulate_business_logic(
    event: &SwapCompleted,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Simular diferentes cenários baseados no user_id
    match event.user_id % 10 {
        // 10% dos casos: simular erro de validação
        0 => {
            warn!(
                "⚠️  Validation warning for user {}: Small amount detected",
                event.user_id
            );
            if event.amount_in < 1.0 {
                return Err("Amount too small for processing".into());
            }
        }
        // 10% dos casos: simular processamento lento
        1 => {
            info!(
                "⏳ Slow processing detected for user {}, simulating delay...",
                event.user_id
            );
            std::thread::sleep(Duration::from_millis(500));
        }
        // 10% dos casos: simular erro de rede/DB (retry)
        2 if event.transaction_id.contains("fail") => {
            error!("💥 Simulated database error for user {}", event.user_id);
            return Err("Database connection failed - retry needed".into());
        }
        // Casos normais
        _ => {
            // Simular cálculo de pontos baseado no volume
            let points = calculate_loyalty_points(event.amount_in, &event.token_in);
            info!(
                "💰 Calculated {} loyalty points for user {}",
                points, event.user_id
            );

            // Simular salvamento no banco
            simulate_database_save(event.user_id, points)?;
        }
    }

    Ok(())
}

/// Simula cálculo de pontos de fidelidade
fn calculate_loyalty_points(amount: f64, token: &str) -> u64 {
    let multiplier = match token {
        "BTC" => 10.0,
        "ETH" => 8.0,
        "USDT" | "USDC" => 1.0,
        _ => 2.0,
    };

    (amount * multiplier) as u64
}

/// Simula operação de salvamento no banco de dados
fn simulate_database_save(
    user_id: u64,
    points: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Simular latência de DB
    std::thread::sleep(Duration::from_millis(50));

    // Simular falha ocasional de DB (5% dos casos em user_ids terminados em 7)
    if user_id % 20 == 7 {
        return Err("Database timeout - connection lost".into());
    }

    info!(
        "💾 Saved {} points to database for user {}",
        points, user_id
    );
    Ok(())
}

/// Publica eventos de teste continuamente para simular tráfego real
async fn event_publisher(client: MessagingClient) -> Result<(), Box<dyn std::error::Error>> {
    info!("📡 Starting event publisher for testing...");

    let tokens = vec![
        ("BTC", "USDT"),
        ("ETH", "USDC"),
        ("BTC", "ETH"),
        ("USDT", "BTC"),
        ("USDC", "ETH"),
        ("ETH", "BTC"),
        ("LINK", "USDT"),
        ("UNI", "ETH"),
        ("AAVE", "USDC"),
    ];

    let mut event_id = 1u64;

    loop {
        // Publicar entre 1-5 eventos por batch
        let batch_size = (event_id % 5) + 1;
        let mut events = Vec::new();

        for i in 0..batch_size {
            let token_pair = &tokens[(event_id + i) as usize % tokens.len()];
            let user_id = 1000 + (event_id + i) % 100; // Users 1000-1099
            let amount = 10.0 + ((event_id + i) % 1000) as f64 / 10.0; // Amounts 10.0-109.9

            // Ocasionalmente criar transações que vão falhar (para testar error handling)
            let tx_id = if (event_id + i) % 50 == 0 {
                format!("fail_tx_{:016x}", event_id + i)
            } else {
                format!("tx_{:016x}", event_id + i)
            };

            let event = SwapCompleted {
                user_id,
                amount_in: amount,
                token_in: token_pair.0.to_string(),
                token_out: token_pair.1.to_string(),
                transaction_id: tx_id,
            };

            events.push(event);
        }

        match client.publish_batch(&events).await {
            Ok(_) => {
                info!(
                    "📤 Published batch of {} events (starting from event #{})",
                    batch_size, event_id
                );
            }
            Err(e) => {
                error!("❌ Failed to publish events: {}", e);
            }
        }

        event_id += batch_size;

        // Intervalo variável entre batches (2-8 segundos)
        let delay = 2000 + (event_id % 6) * 1000;
        sleep(Duration::from_millis(delay)).await;
    }
}
