use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::sleep;
use tokio_serial::SerialStream;
use zstacker_znp_protocol::commands::START_OF_FRAME;
use zstacker_znp_protocol::framing::CommandMeta;

pub mod responses;

pub async fn mock_adaptor(mut serial: SerialStream) {
    loop {
        let mut buf = [0u8; 4];
        serial.read_exact(&mut buf).await.unwrap();
        let [START_OF_FRAME, data_length, meta @ ..] = buf else {
            panic!("frame should start with start of frame");
        };
        let meta = CommandMeta::deserialize(meta).unwrap();
        let mut data = vec![0u8; data_length as usize + 1];
        serial.read_exact(&mut data).await.unwrap();

        match meta {
            responses::RESET => {
                serial.write_all(&responses::reset()).await.unwrap();
            }
            responses::GET_DEVICE_INFO => {
                serial.write_all(&responses::device_info()).await.unwrap();
            }
            responses::SYS_VERSION => {
                serial.write_all(&responses::sys_version()).await.unwrap();
            }
            responses::EXT_FIND_GROUP => {
                serial.write_all(&responses::find_group()).await.unwrap();
            }
            responses::LQI_REQ => {
                serial.write_all(&responses::lqi_status()).await.unwrap();
                sleep(Duration::from_millis(300)).await;
                serial.write_all(&responses::lqi()).await.unwrap();
            }
            CommandMeta { .. } => {
                panic!("mock can not handle command type: {meta:?}")
            }
        }
    }
}
