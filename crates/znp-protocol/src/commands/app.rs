// #[derive(Debug, Clone, Serialize)]
// pub struct Msg {
//     pub appendpoint: u8,
//     pub destaddress: u16,
//     pub destendpoint: u8,
//     pub clusterid: u16,
//     pub msglen: u8,
//     pub message: Vec<u8>,
// }
//
// impl SyncRequest for Msg {
//     const ID: u8 = 0;
//     const SUBSYSTEM: SubSystem = SubSystem::App;
//     type Reply = MsgReply;
// }
//
// basic_reply! { Msg, MsgReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct UserTest {
//     pub srcep: u8,
//     pub commandid: u16,
//     pub param1: u16,
//     pub param2: u16,
// }
//
// impl SyncRequest for UserTest {
//     const ID: u8 = 1;
//     const SUBSYSTEM: SubSystem = SubSystem::App;
//     type Reply = UserTestReply;
// }
//
// basic_reply! { UserTest, UserTestReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct ZllTlInd {
//     pub nwkaddr: u16,
//     pub endpoint: u8,
//     pub profileid: u16,
//     pub deviceid: u16,
//     pub version: u8,
// }
//
// impl AsyncRequest for ZllTlInd {
//     const ID: u8 = 129;
//     const SUBSYSTEM: SubSystem = SubSystem::App;
// }
