#![allow(dead_code)]

// use serde::{Deserialize, Serialize};
// use serde_repr::Serialize_repr;
//
// use super::{
//     basic_reply, AsyncRequest, IeeeAddr, SubSystem, SyncReply, SyncRequest
// };
//
// #[derive(Clone, Copy, Debug, Serialize_repr)]
// #[repr(u8)]
// pub enum InstallCodeFormat {
//     InstallCodePlusCRC = 0x01,
//     KeyDerivedFromInstallCode = 0x02,
// }
//
// #[derive(Debug, Clone)]
// pub struct AddInstallCode {
//     pub format: InstallCodeFormat,
//     pub addr: IeeeAddr,
//     pub code: Vec<u8>,
// }
//
// impl Serialize for AddInstallCode {
//     fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         unimplemented!(
//             "can not be automatically serialized. Instead uses custom data_to_vec"
//         )
//     }
// }
//
// impl SyncRequest for AddInstallCode {
//     const ID: u8 = 4;
//     const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
//     type Reply = AddInstallCodeReply;
//
//     fn data_to_vec(&self) -> Result<Vec<u8>, crate::data_format::Error>
//     where
//         Self: Sized,
//     {
//         let code = match self.format {
//             InstallCodeFormat::InstallCodePlusCRC => {
//                 self.code.iter().copied().take(usize::MAX)
//             }
//             InstallCodeFormat::KeyDerivedFromInstallCode => {
//                 self.code.iter().copied().take(16)
//             }
//         };
//
//         Ok(std::iter::once(self.format as u8)
//             .chain(self.addr.0.to_le_bytes())
//             .chain(code)
//             .collect())
//     }
// }
//
//
// basic_reply! { AddInstallCode, AddInstallCodeReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct BdbStartCommissioning {
//     pub mode: u8,
// }
//
// impl SyncRequest for BdbStartCommissioning {
//     const ID: u8 = 5;
//     const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
//     type Reply = BdbStartCommissioningReply;
// }
//
//
// basic_reply! { BdbStartCommissioning, BdbStartCommissioningReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct BdbSetChannel {
//     pub is_primary: u8,
//     pub channel: u32,
// }
//
// impl SyncRequest for BdbSetChannel {
//     const ID: u8 = 8;
//     const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
//     type Reply = BdbSetChannelReply;
// }
//
//
// basic_reply! { BdbSetChannel, BdbSetChannelReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct BdbSetTcRequireKeyExchange {
//     pub value: u8,
// }
// impl SyncRequest for BdbSetTcRequireKeyExchange {
//     const ID: u8 = 9;
//     const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
//     type Reply = BdbSetTcRequireKeyExchangeReply;
// }
//
//
// basic_reply! { BdbSetTcRequireKeyExchange, BdbSetTcRequireKeyExchangeReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct BdbComissioningNotification {
//     pub status: u8,
// }
//
// impl AsyncRequest for BdbComissioningNotification {
//     const ID: u8 = 128;
//     const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
// }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct SetNwkFrameCounter {
//     pub value: u32,
// }
//
// impl SyncRequest for SetNwkFrameCounter {
//     const ID: u8 = 255;
//     const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
//     type Reply = SetNwkFrameCounterReply;
// }
//
//
// basic_reply! { SetNwkFrameCounter, SetNwkFrameCounterReply }
