use serde::Serialize;

use super::{Command, CommandType, Status, Subsystem};

#[derive(Debug, Clone, Serialize)]
struct Msg {
	appendpoint: u8,
	destaddress: u16,
	destendpoint: u8,
	clusterid: u16,
	msglen: u8,
	message: Vec<u8>,
}

impl Command for Msg {
	const ID: u8 = 0;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::App;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct UserTest {
	srcep: u8,
	commandid: u16,
	param1: u16,
	param2: u16,
}

impl Command for UserTest {
	const ID: u8 = 1;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::App;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ZllTlInd {
	nwkaddr: u16,
	endpoint: u8,
	profileid: u16,
	deviceid: u16,
	version: u8,
}

impl Command for ZllTlInd {
	const ID: u8 = 129;
	const TYPE: CommandType = CommandType::AREQ;
	const SUBSYSTEM: Subsystem = Subsystem::App;
	type Reply = ();
}

