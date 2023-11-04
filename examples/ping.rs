use std::time::Duration;

use mt_interface::{data::MtCommand, wire::GeneralSerialPacket};
use serialport::SerialPort;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/cu.usbserial-14320", 115_200)
        .timeout(Duration::from_millis(10000))
        .open()
        .expect("Failed to open port");

    let commands = vec![
        MtCommand::sys_ping(),
        MtCommand::sys_version(),
        MtCommand::sys_get_extaddr(),
        MtCommand::sys_osal_start_timer(3, 1500),
        MtCommand::sys_osal_stop_timer(3),
        MtCommand::sys_random(),
        MtCommand::util_get_device_info(),
        MtCommand::util_get_nv_info(),
        MtCommand::util_time_alive(),
        MtCommand::util_srng_gen(),
    ];

    for command in commands {
        send_command(&mut port, command, true);
    }
}

fn send_command(port: &mut Box<dyn SerialPort>, mt_cmd: MtCommand, should_read:bool) {
    let packet = GeneralSerialPacket::from_cmd(mt_cmd);
    let tx = packet.to_frame();
        print!("Write: ");
        print_packet(&tx, tx.len());
        let start = std::time::Instant::now();
        port.write(tx.as_slice()).expect("Write failed!");

        if should_read {
            let mut serial_buf: Vec<u8> = vec![0; 100];
            let len = port
                .read(serial_buf.as_mut_slice())
                .expect("Found no data!");
            
            print!("Read: ");
            print_packet(&serial_buf, len);
        }
        let end = std::time::Instant::now();
        print!("Command took: {}ms", (end - start).as_millis());
        println!();
}

fn print_packet(packet: &Vec<u8>, len: usize) {
    let n = packet.len().min(len);

    for i in 0..n {
        print!("{:02x} ", packet[i]);
    }
    println!();
}
