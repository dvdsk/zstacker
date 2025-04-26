use std::time::Duration;

use color_eyre::eyre::{Context, eyre};
use tokio::time::sleep;
use tokio_serial::{SerialPortBuilderExt, SerialPortType};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};
use zstacker_znp::coordinator::Adaptor;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()?,
        )
        .with(fmt::layer().pretty().with_line_number(true))
        .try_init()?;

    let ports = tokio_serial::available_ports()
        .expect("No ports found!")
        .into_iter()
        .filter(|p| matches!(p.port_type, SerialPortType::UsbPort(_)));

    let mut adaptor = None;
    for p in ports {
        let mut candidate = tokio_serial::new(p.port_name.clone(), 115_200)
            .timeout(Duration::from_secs(90))
            .open_native_async()
            .expect("Failed to open port");
        candidate.set_exclusive(true).unwrap();
        let mut candidate = Adaptor::start(candidate);
        match zstacker_znp::check_connection_to_adapter(&mut candidate).await {
            Ok(()) => adaptor = Some(candidate),
            Err(e) => eprintln!(
                "error while connecting: \n\tport: {}\n\terror: {e:?}\ncould be wrong port?",
                p.port_name
            ),
        }
    }

    let Some(adaptor) = adaptor else {
        return Err(eyre!("No adapter found"));
    };
    let mut coordinator =
        zstacker_znp::start_coordinator(adaptor, vec![], true)
            .await
            .wrap_err("Could not start coordinator")?;
    println!("sleeping to give the coordinator time to discover the network");
    sleep(Duration::from_secs(60)).await;
    let table = coordinator
        .lqi_table()
        .await
        .wrap_err("Could not get active adresses")?;
    dbg!(table);
    Ok(())
}
