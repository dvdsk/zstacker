use serde::{Deserialize, Serialize};
use zstacker_znp_protocol::commands::IeeeAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NwkKeyDesc {
    key_seq_num: u8,
    key: Vec<u8>,
}

/// https://github.com/zigpy/zigpy-znp/blob/dev/zigpy_znp/types/structs.py#L73
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Nib {
    sequence_num: u8,
    passive_ack_timeout: u8,
    max_broadcast_retries: u8,
    max_children: u8,
    max_depth: u8,
    max_routers: u8,
    dummy_neighbor_table: u8,
    broadcast_delivery_time: u8,
    report_constant_cost: u8,
    route_disc_retries: u8,
    dummy_routing_table: u8,
    secure_all_frames: u8,
    security_level: u8,
    sym_link: u8,
    capability_flags: u8,

    transaction_persistence_time: u16,

    nwk_protocol_version: u8,
    route_discovery_time: u8,
    route_expiry_time: u8,

    nwk_dev_address: u16,

    nwk_logical_channel: u8,

    nwk_coord_address: u16,
    nwk_coord_ext_address: IeeeAddr,
    nwk_pan_id: u16,

    nwk_state: u16,
    channel_list: u32,

    beacon_order: u8,
    super_frame_order: u8,
    scan_duration: u8,
    batt_life_ext: u8,

    allocated_router_addresses: u32,
    allocated_end_device_addresses: u32,

    node_depth: u8,

    extended_panid: u64,

    nwk_key_loaded: bool,

    spare1: NwkKeyDesc,
    spare2: NwkKeyDesc,

    spare3: u8,
    spare4: u8,

    nwk_link_status_period: u8,
    nwk_router_age_limit: u8,
    nwk_use_multi_cast: bool,
    nwk_is_concentrator: bool,
    nwk_concentrator_discovery_time: u8,
    nwk_concentrator_radius: u8,
    nwk_all_fresh: u8,

    nwk_manager_addr: u16,
    nwk_total_transmissions: u16,
    nwk_update_id: u8,
}
