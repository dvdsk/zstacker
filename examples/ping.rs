use std::time::Duration;

use mt_interface::{data::MtCommand, wire::GeneralSerialPacket};

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/cu.usbserial-14320", 115_200)
        .timeout(Duration::from_millis(10000))
        .open()
        .expect("Failed to open port");

    let packets = vec![
        GeneralSerialPacket::from_cmd(MtCommand::sys_ping()),
        GeneralSerialPacket::from_cmd(MtCommand::sys_version()),
        GeneralSerialPacket::from_cmd(MtCommand::sys_get_extaddr()),
        GeneralSerialPacket::from_cmd(MtCommand::sys_osal_start_timer(3, 1500)),
        GeneralSerialPacket::from_cmd(MtCommand::sys_osal_stop_timer(3)),
        GeneralSerialPacket::from_cmd(MtCommand::sys_random()),
        GeneralSerialPacket::from_cmd(MtCommand::util_get_device_info()),
        GeneralSerialPacket::from_cmd(MtCommand::util_get_nv_info()),
        GeneralSerialPacket::from_cmd(MtCommand::util_time_alive()),
        GeneralSerialPacket::from_cmd(MtCommand::util_srng_gen()),
    ];

    for packet in packets {
        let tx = packet.to_frame();
        print!("Write: ");
        print_packet(&tx, tx.len());
        let start = std::time::Instant::now();
        port.write(tx.as_slice()).expect("Write failed!");

        let mut serial_buf: Vec<u8> = vec![0; 100];
        let len = port
            .read(serial_buf.as_mut_slice())
            .expect("Found no data!");
        let end = std::time::Instant::now();

        print!("Read: ");
        print_packet(&serial_buf, len);
        print!("Command took: {}ms", (end - start).as_millis());
        println!();
    }
}

fn print_packet(packet: &Vec<u8>, len: usize) {
    let n = packet.len().min(len);

    for i in 0..n {
        print!("{:02x} ", packet[i]);
    }
    println!();
}
