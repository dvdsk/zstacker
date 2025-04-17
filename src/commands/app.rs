#![allow(dead_code)]

use serde::Serialize;

use super::{AsyncRequest, SyncRequest, Status, SubSystem};

#[derive(Debug, Clone, Serialize)]
struct Msg {
	appendpoint: u8,
	destaddress: u16,
	destendpoint: u8,
	clusterid: u16,
	msglen: u8,
	message: Vec<u8>,
}

impl SyncRequest for Msg {
	const ID: u8 = 0;
	const SUBSYSTEM: SubSystem = SubSystem::App;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct UserTest {
	srcep: u8,
	commandid: u16,
	param1: u16,
	param2: u16,
}

impl SyncRequest for UserTest {
	const ID: u8 = 1;
	const SUBSYSTEM: SubSystem = SubSystem::App;
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

impl AsyncRequest for ZllTlInd {
	const ID: u8 = 129;
	const SUBSYSTEM: SubSystem = SubSystem::App;
}

