use futures_concurrency::future::Race;
use tokio_serial::SerialStream;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};
use zstacker_test_support::mock_adaptor;
use zstacker_znp::coordinator::Adaptor;
use zstacker_znp::start_coordinator;

async fn run_test(serial: SerialStream) {
    let adaptor = Adaptor::start(serial);
    let mut coordinator = start_coordinator(adaptor, Vec::new(), false).await.unwrap();
    coordinator.list_addresses_on_network().await.unwrap();
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()?,
        )
        .with(fmt::layer())
        .try_init()?;

    let (b, a) = SerialStream::pair().unwrap();
    (mock_adaptor(a), run_test(b)).race().await;

    Ok(())
}
