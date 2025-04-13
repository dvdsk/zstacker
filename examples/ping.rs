use std::time::Duration;

use mt_interface::{
    constants::{
        CommissioningMode, DeviceSpecificConfigurationItem, LatencyReq, NvStartupOptionBitMask,
        ResetRequestType,
    },
    data::MtCommand,
    wire::GeneralSerialPacket,
};
use serialport::SerialPort;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_millis(10000))
        .open()
        .expect("Failed to open port");

    let commands = startup_sequence();
    for (desc, command, should_read, delay) in commands {
        send_command(&mut port, command, should_read, desc, delay);
    }

    loop {
        if let Ok((serial_buf, len)) = try_read(&mut port) {
            let ms = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0);
            print!("Read {}: ", ms);
            print_packet(&serial_buf, len);
        }
    }
}

fn try_read(port: &mut Box<dyn SerialPort>) -> Result<(Vec<u8>, usize), serialport::Error> {
    let mut serial_buf: Vec<u8> = vec![0; 100];
    let len = port.read(serial_buf.as_mut_slice())?;

    Ok((serial_buf, len))
}

fn print_read(port: &mut Box<dyn SerialPort>) {
    let res = try_read(port);
    match res {
        Err(_) => println!("Error reading!"),
        Ok((serial_buf, len)) => {
            print!("Read: ");
            print_packet(&serial_buf, len);
        }
    }
}

fn send_command(
    port: &mut Box<dyn SerialPort>,
    mt_cmd: MtCommand,
    should_read: bool,
    desc: &str,
    delay: u64,
) {
    let packet = GeneralSerialPacket::from_cmd(mt_cmd);
    let tx = packet.to_frame();
    println!("####{}", desc);
    print!("Write: ");
    print_packet(&tx, tx.len());
    let start = std::time::Instant::now();
    port.write(tx.as_slice()).expect("Write failed!");

    if should_read {
        print_read(port);
    }
    if delay > 0 {
        println!("Sleeping for {}ms", delay);
        std::thread::sleep(Duration::from_millis(delay));
        print_read(port);
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

fn get_occupancy_consumer() -> (&'static str, MtCommand, bool, u64) {
    let mut occupancy_sense_cluster: [u16; 16] = [0; 16];
    occupancy_sense_cluster[0] = 0x0406;

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
            occupancy_sense_cluster,
        ),
        true,
        0
    )
}

fn get_start_comssioning_formation() -> (&'static str, MtCommand, bool, u64) {
    (
        "BDB Start Commissioning - Network Formation ",
        MtCommand::app_cnf_bdb_start_commissioning(CommissioningMode::NetworkFormation),
        true,
        0,
    )
}

fn startup_sequence() -> Vec<(&'static str, MtCommand, bool, u64)> {
    let clear_conf: [u8; 1] = [NvStartupOptionBitMask::ZcdStartoptClearConfig as u8
        | NvStartupOptionBitMask::ZcdStartoptClearNwkFrameCounter as u8
        | NvStartupOptionBitMask::ZcdStartoptClearState as u8];
    vec![
        // (
        //     "Reset device",
        //     MtCommand::sys_reset_req(ResetRequestType::SoftReset),
        //     true,
        //     1000,
        // ),
        // (
        //     "Clear config",
        //     MtCommand::sys_osal_nv_write(
        //         DeviceSpecificConfigurationItem::ZcdNvStartupOption as u16,
        //         0,
        //         1,
        //         &clear_conf,
        //     ),
        //     true,
        //     0,
        // ),
        // (
        //     "Reset device",
        //     MtCommand::sys_reset_req(ResetRequestType::SoftReset),
        //     true,
        //     1000,
        // ),
        // get_occupancy_consumer(),
        // get_start_comssioning_formation(),
    ]
}

fn testing_startup() -> Vec<(&'static str, MtCommand, bool, u64)> {
    vec![
        // ("Ping", MtCommand::sys_ping(), true),
        // ("Version", MtCommand::sys_version(), true),
        // ("Extended Address", MtCommand::sys_get_extaddr(), true),
        // (
        //     "Start Timer",
        //     MtCommand::sys_osal_start_timer(3, 0xabcd),
        //     true,
        // ),
        // ("Stop Timer", MtCommand::sys_osal_stop_timer(3), true),
        // ("Random", MtCommand::sys_random(), true),
        ("Device Info", MtCommand::util_get_device_info(), true, 0),
        // ("NV Info", MtCommand::util_get_nv_info(), true),
        // ("Time Alive", MtCommand::util_time_alive(), true),
        // ("SRNG Gen", MtCommand::util_srng_gen(), true),
        // (
        //     "Nv Read logical type",
        //     MtCommand::sys_osal_nv_read(
        //         DeviceSpecificConfigurationItem::ZCD_NV_LOGICAL_TYPE as u16,
        //         0,
        //     ),
        //     true,
        // ),
        // (
        //     "Nv Read direct db",
        //     MtCommand::sys_osal_nv_read(
        //         DeviceSpecificConfigurationItem::ZCD_NV_ZDO_DIRECT_CB as u16,
        //         0,
        //     ),
        //     true,
        // ),
        // (
        //     "Nv Read startup option",
        //     MtCommand::sys_osal_nv_read(
        //         DeviceSpecificConfigurationItem::ZCD_NV_STARTUP_OPTION as u16,
        //         0,
        //     ),
        //     true,
        // ),
        // (
        //     "Nv Read PAN-ID",
        //     MtCommand::sys_osal_nv_read(NetworkSpecificConfigurationItem::ZCD_NV_PANID as u16, 0),
        //     true,
        // ),
        get_occupancy_consumer(),
        // (
        //     "Discovery all channels",
        //     MtCommand::zdo_nwk_discovery_req(ScanChannels::AllChannels, 0x01),
        //     true,
        // ),

        // (
        //     "BDB set primary channel to Channel 11",
        //     MtCommand::app_cnf_bdb_set_channel(true, ScanChannels::Channel11),
        //     true,
        // ),
        // (
        //     "BDB set secondary channel to Channel 20",
        //     MtCommand::app_cnf_bdb_set_channel(false, ScanChannels::Channel20),
        //     true,
        // ),
        // (
        //     "BDB set bdbTrustCenterRequireKeyExchange to false",
        //     MtCommand::app_cnf_bdb_set_tc_require_key_exchange(false),
        //     true,
        // ),
        // (
        //     "BDB set bdbJoinUsesInstallCodeKey to false",
        //     MtCommand::app_cnf_bdb_set_joinusesinstallcodekey(false),
        //     true,
        // ),
        get_start_comssioning_formation(),
        ("Nv NIB", MtCommand::sys_osal_nv_read(0x0021, 0), true, 0),
    ]
}
