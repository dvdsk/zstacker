use std::io::Write;

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
use zstacker_znp_protocol::commands::sys::{ResetInd, ResetReason};
use zstacker_znp_protocol::commands::{
    AsyncReply, CommandType, START_OF_FRAME, SubSystem, to_frame,
};
use zstacker_znp_protocol::data_format;
use zstacker_znp_protocol::framing::CommandMeta;

const RESET: CommandMeta = CommandMeta {
    ty: CommandType::AREQ,
    sub_system: SubSystem::Sys,
    id: 0,
};

fn reset_response() -> Vec<u8> {
    const RESET_RESPONSE: ResetInd = ResetInd {
        reason: ResetReason::External,
        transport_rev: 0,
        product_id: 0,
        major_rel: 0,
        minor_rel: 0,
        hw_rev: 0,
    };

    to_frame(
        data_format::to_vec(&RESET_RESPONSE).unwrap(),
        ResetInd::META,
    )
    .unwrap()
}

async fn mock_adaptor(mut serial: SerialStream) {
    loop {
        let mut buf = [0u8; 4];
        serial.read_exact(&mut buf).await.unwrap();
        let [START_OF_FRAME, data_length, meta @ ..] = buf else {
            panic!("frame should start with start of frame");
        };
        dbg!();
        let meta = CommandMeta::deserialize(meta).unwrap();
        let mut data = vec![0u8; data_length as usize + 1];
        serial.read_exact(&mut data).await.unwrap();
        dbg!();

        match dbg!(&meta) {
            &RESET => {
                serial.write_all(&reset_response()).unwrap();
                debug!("wrote {} bytes", reset_response().len());
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
