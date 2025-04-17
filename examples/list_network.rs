use std::time::Duration;

use color_eyre::eyre::{Context, eyre};
use serialport::SerialPortType;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

struct UsbSerial(Box<dyn serialport::SerialPort>);

impl std::fmt::Debug for UsbSerial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UsbSerial")
            .field("port:", &self.0.name())
            .finish()
    }
}

impl std::io::Read for UsbSerial {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl std::io::Write for UsbSerial {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl zstacker::api::Serial for UsbSerial {}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()?,
        )
        .with(fmt::layer())
        .try_init()?;

    let ports = serialport::available_ports()
        .expect("No ports found!")
        .into_iter()
        .filter(|p| matches!(p.port_type, SerialPortType::UsbPort(_)));

    let mut adaptor = None;
    for p in ports {
        let serial = serialport::new("/dev/ttyUSB0", 115_200)
            .timeout(Duration::from_millis(500))
            .open()
            .expect("Failed to open port");
        let serial = UsbSerial(serial);
        let mut candidate = zstacker::api::Adaptor { serial };
        match zstacker::api::check_connection_to_adapter(&mut candidate) {
            Ok(()) => adaptor = Some(candidate),
            Err(e) => eprintln!(
                "error while connecting: \n\tport: {}\n\terror: {e:?}\ncould be wrong port?",
                p.port_name
            ),
        }
    }

    let Some(mut adaptor) = adaptor else {
        return Err(eyre!("No adapter found"));
    };
    zstacker::api::start_coordinator(&mut adaptor)
        .wrap_err("Could not start coordinator")?;
    Ok(())
}

//[
//fe, start of frame
//e,  length
//67, cmd0
//0,  cmd1
//]
//[
//0,  status
//51, |
//13, |
//8b, |
//2f, |
//0,  | IEEEAddr
//4b, |
//12, |
//0,  |
//fe,   | ShortAddr
//ff, *
//7,  devicetype
//0,  devicestate
//0,  list len
//d0  list element
//]
