use zstacker_znp_protocol::commands::{
    AsyncReply, AsyncRequest, CommandType, ShortAddr,
    SubSystem, SyncReply, to_frame,
};
use zstacker_znp_protocol::data_format;
use zstacker_znp_protocol::framing::CommandMeta;

pub(crate) const RESET: CommandMeta = CommandMeta {
    ty: CommandType::AREQ,
    sub_system: SubSystem::Sys,
    id: 0,
};

pub(crate) const GET_DEVICE_INFO: CommandMeta = CommandMeta {
    ty: CommandType::SREQ,
    sub_system: SubSystem::Util,
    id: 0,
};

pub(crate) const SYS_VERSION: CommandMeta = CommandMeta {
    ty: CommandType::SREQ,
    sub_system: SubSystem::Sys,
    id: 2,
};

pub(crate) const EXT_FIND_GROUP: CommandMeta = CommandMeta {
    ty: CommandType::SREQ,
    sub_system: SubSystem::Zdo,
    id: 74,
};

pub(crate) const LQI_REQ: CommandMeta = CommandMeta {
    ty: CommandType::SREQ,
    sub_system: SubSystem::Zdo,
    id: 49,
};

pub(crate) fn reset() -> Vec<u8> {
    use zstacker_znp_protocol::commands::sys::{ResetInd, ResetReason};
    pub(crate) const RESPONSE: ResetInd = ResetInd {
        reason: ResetReason::External,
        transport_rev: 0,
        product_id: 0,
        major_rel: 0,
        minor_rel: 0,
        hw_rev: 0,
    };

    to_frame(data_format::to_vec(&RESPONSE).unwrap(), ResetInd::META).unwrap()
}

pub(crate) fn sys_version() -> Vec<u8> {
    use zstacker_znp_protocol::commands::sys::VersionReply;
    pub(crate) const RESPONSE: VersionReply = VersionReply {
        transportrev: 0,
        product: 1,
        majorrel: 2,
        minorrel: 3,
        maintrel: 4,
        revision: 5,
    };

    to_frame(data_format::to_vec(&RESPONSE).unwrap(), VersionReply::META)
        .unwrap()
}

pub(crate) fn device_info() -> Vec<u8> {
    use zstacker_znp_protocol::commands::util::DeviceInfo;
    use zstacker_znp_protocol::commands::{
        DeviceState, DeviceType, IeeeAddr, ShortAddr,
    };
    let response = DeviceInfo {
        status: 0,
        ieee_addr: IeeeAddr(42u64),
        short_addr: ShortAddr(43u16),
        can_operate_as: vec![DeviceType::Coordinator, DeviceType::EndDevice],
        device_state: DeviceState::StartedAsZBCoordinator,
        assoc_devices: Vec::new(),
    };
    to_frame(data_format::to_vec(&response).unwrap(), DeviceInfo::META).unwrap()
}

pub(crate) fn find_group() -> Vec<u8> {
    use zstacker_znp_protocol::commands::zdo::ExtFindGroupReply;
    let response = ExtFindGroupReply {
        group_info: [0u8; 18],
    };

    to_frame(
        data_format::to_vec(&response).unwrap(),
        ExtFindGroupReply::META,
    )
    .unwrap()
}

pub(crate) fn lqi_status() -> Vec<u8> {
    use zstacker_znp_protocol::commands::BasicStatus;
    use zstacker_znp_protocol::commands::zdo::MgmtLqiReq;
    to_frame(
        data_format::to_vec(&BasicStatus::Ok).unwrap(),
        MgmtLqiReq::status_reply_meta().unwrap(),
    )
    .unwrap()
}

pub(crate) fn lqi() -> Vec<u8> {
    use zstacker_znp_protocol::commands::zdo::{MgmtLqiRsp, NeighborLqi};
    use zstacker_znp_protocol::commands::{BasicStatus, IeeeAddr};
    to_frame(
        data_format::to_vec(&MgmtLqiRsp {
            srcaddr: ShortAddr(43),
            status: BasicStatus::Ok,
            neighbortable_entries: 1,
            startindex: 0,
            neighbor_lqi_list: vec![NeighborLqi {
                extended_pan_id: 0,
                extended_address: IeeeAddr(2),
                network_address: ShortAddr(2),
                device_type:
                    zstacker_znp_protocol::commands::DeviceType::Router,
                rx_on_when_idle: 1,
                relationship: 1,
                permit_joining: true,
                depth: 1,
                lqi: 16,
            }],
        })
        .unwrap(),
        MgmtLqiRsp::META,
    )
    .unwrap()
}
