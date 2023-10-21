use std::time::Duration;

use mt_interface::{data::MtCommand, wire::GeneralSerialPacket};

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/cu.usbserial-14320", 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    let mt_cmd = MtCommand::sys_ping();
    let packet = GeneralSerialPacket::from_cmd(mt_cmd);
    port.write(packet.to_frame().as_slice())
        .expect("Write failed!");

    let mut serial_buf: Vec<u8> = vec![0; 10];
    port.read(serial_buf.as_mut_slice())
        .expect("Found no data!");

    println!("Read: ");
    for byte in &serial_buf {
        print!("{:02x} ", byte);
    }
    println!();
}
