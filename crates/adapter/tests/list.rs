use futures_concurrency::future::Race;
use tokio::io::AsyncReadExt;
use tokio_serial::SerialStream;
use tracing::debug;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};
use zstacker_znp::coordinator::Adaptor;
use zstacker_znp::start_coordinator;
use zstacker_znp_protocol::commands::{CommandType, START_OF_FRAME, SubSystem};
use zstacker_znp_protocol::framing::CommandMeta;

async fn mock_adaptor(mut serial: SerialStream) {
    loop {
        dbg!();
        assert_eq!(serial.read_u8().await.unwrap(), START_OF_FRAME);
        let mut buf = [0u8; 3];
        serial.read_exact(&mut buf).await.unwrap();
        let [length, meta @ ..] = buf;
        let meta = CommandMeta::deserialize(meta).unwrap();
        let mut data = vec![0u8; length as usize + 1];
        serial.read_exact(&mut data).await.unwrap();

        match meta {
            CommandMeta {
                ty: CommandType::AREQ,
                sub_system: SubSystem::Sys,
                id: 0,
            } => {
                debug!("mock reset");
            }
            CommandMeta { .. } => {
                panic!("mock can not handle command type: {meta:?}")
            }
        }
    }
}

async fn run_test(serial: SerialStream) {
    let adaptor = Adaptor::start(serial);
    let mut coordinator = start_coordinator(adaptor, Vec::new()).await.unwrap();
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
