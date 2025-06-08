use bullpen_dex_messaging::{
    config::NatsConfig, model::dexevents::PriceUpdate, prelude::NatsMessagingClient,
    traits::Realtime, DexMessagingResult,
};
use dotenv::dotenv;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("🚀 Starting NATS Event Simulator...");
    let messaging_client = Arc::new(NatsMessagingClient::new(NatsConfig::from_env()?).await?);
    info!("✅ Connected to NATS and Iggy successfully");

    // Spawn price publisher task
    let publisher_client = messaging_client.clone();
    tokio::spawn(async move {
        if let Err(e) = price_publisher(publisher_client).await {
            error!("Price publisher failed: {}", e);
        }
    });

    // Spawn swap events publisher task
    let publisher_client = messaging_client.clone();
    tokio::spawn(async move {
        if let Err(e) = price_publisher(publisher_client).await {
            error!("Swap publisher failed: {}", e);
        }
    });

    // Price subscriber with simple logging
    info!("🎯 Starting price update subscriber for all tokens...");

    let price_handler = |price_update: PriceUpdate| -> DexMessagingResult<()> {
        info!(
            "💰 Price Update: {} = ${:.4} (timestamp: {})",
            price_update.token_pair, price_update.price, price_update.timestamp
        );

        // Simulate different scenarios based on price changes
        if price_update.price > 100000.0 {
            warn!(
                "🚨 High price alert for {}: ${:.2}",
                price_update.token_pair, price_update.price
            );
        }

        Ok(())
    };

    // Subscribe to all price updates using NATS wildcard
    messaging_client
        .subscribe::<PriceUpdate, _>("prices.*", price_handler)
        .await?;

    Ok(())
}

/// Publishes realistic price updates for various trading pairs
async fn price_publisher(
    client: Arc<NatsMessagingClient>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("📡 Starting price publisher...");

    // Define trading pairs with base prices
    let trading_pairs = vec![
        ("BTC-USD", 45000.0),
        ("ETH-USD", 3200.0),
        ("BNB-USD", 320.0),
        ("ADA-USD", 0.85),
        ("SOL-USD", 110.0),
        ("MATIC-USD", 1.2),
        ("LINK-USD", 18.5),
        ("UNI-USD", 12.0),
        ("AAVE-USD", 180.0),
        ("DOT-USD", 8.5),
    ];

    let mut current_prices: Vec<f64> = trading_pairs.iter().map(|(_, price)| *price).collect();
    let mut counter = 0u64;

    loop {
        for (i, (pair, _)) in trading_pairs.iter().enumerate() {
            // Use deterministic price movements based on counter
            let change_percent = (counter as f64 * 0.01).sin() * 0.02;
            current_prices[i] *= 1.0 + change_percent;

            // Ensure prices don't go negative
            if current_prices[i] < 0.01 {
                current_prices[i] = 0.01;
            }

            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let price_update = PriceUpdate {
                token_pair: pair.to_string(),
                price: current_prices[i],
                timestamp,
            };

            match client.publish(&price_update).await {
                Ok(_) => {
                    if i == 0 {
                        // Log only BTC for brevity
                        info!("📤 Published price: {} = ${:.2}", pair, current_prices[i]);
                    }
                }
                Err(e) => error!("❌ Failed to publish price update for {}: {}", pair, e),
            }

            // Small delay between individual price updates
            sleep(Duration::from_millis(100)).await;
        }

        counter = counter.wrapping_add(1);

        // Fixed delay between price update cycles (3 seconds)
        sleep(Duration::from_secs(3)).await;
    }
}
