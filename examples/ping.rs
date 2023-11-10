use std::time::Duration;

use mt_interface::{
    constants::{DeviceSpecificConfigurationItem, LatencyReq, NetworkSpecificConfigurationItem},
    data::MtCommand,
    wire::GeneralSerialPacket,
};
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

    let mut OCCUPANCY_SENSE_CLUSTER: [u16; 16] = [0; 16];
    OCCUPANCY_SENSE_CLUSTER[0] = 0x0406;
    let commands = vec![
        ("Ping", MtCommand::sys_ping(), true),
        ("Version", MtCommand::sys_version(), true),
        ("Extended Address", MtCommand::sys_get_extaddr(), true),
        (
            "Start Timer",
            MtCommand::sys_osal_start_timer(3, 0xabcd),
            true,
        ),
        ("Stop Timer", MtCommand::sys_osal_stop_timer(3), true),
        ("Random", MtCommand::sys_random(), true),
        ("Device Info", MtCommand::util_get_device_info(), true),
        ("NV Info", MtCommand::util_get_nv_info(), true),
        ("Time Alive", MtCommand::util_time_alive(), true),
        ("SRNG Gen", MtCommand::util_srng_gen(), true),
        (
            "Nv Read logical type",
            MtCommand::sys_osal_nv_read(
                DeviceSpecificConfigurationItem::ZCD_NV_LOGICAL_TYPE as u16,
                0,
            ),
            true,
        ),
        (
            "Nv Read direct db",
            MtCommand::sys_osal_nv_read(
                DeviceSpecificConfigurationItem::ZCD_NV_ZDO_DIRECT_CB as u16,
                0,
            ),
            true,
        ),
        (
            "Nv Read startup option",
            MtCommand::sys_osal_nv_read(
                DeviceSpecificConfigurationItem::ZCD_NV_STARTUP_OPTION as u16,
                0,
            ),
            true,
        ),
        (
            "Nv Read PAN-ID",
            MtCommand::sys_osal_nv_read(NetworkSpecificConfigurationItem::ZCD_NV_PANID as u16, 0),
            true,
        ),
        (
            "Register endpoint 1, prof id 0x0104, dev id 0x0100 ver 1, no input clusters, output cluster: 0x0406",
            MtCommand::af_register(
                0x01,
                0x0104,
                0x0100,
                0x01,
                LatencyReq::NoLatency,
                0x00,
                [0;16],
                0x01,
                OCCUPANCY_SENSE_CLUSTER,
            ),
            true,
        ),
    ];

    for (desc, command, should_read) in commands {
        send_command(&mut port, command, should_read, desc);
    }
}

fn send_command(port: &mut Box<dyn SerialPort>, mt_cmd: MtCommand, should_read: bool, desc: &str) {
    let packet = GeneralSerialPacket::from_cmd(mt_cmd);
    let tx = packet.to_frame();
    println!("####{}", desc);
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
    println!("Command took: {}ms", (end - start).as_millis());
    println!("####");
}

fn print_packet(packet: &Vec<u8>, len: usize) {
    let n = packet.len().min(len);

    for i in 0..n {
        print!("{:02x} ", packet[i]);
    }
    println!();
}
