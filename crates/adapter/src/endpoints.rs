use zstacker_znp_protocol::commands::af::{
    ClusterId, LatencyRequirement, Register,
};

const SS_IAS_ZONE: ClusterId = ClusterId(1280);
const SS_IAS_ACE: ClusterId = ClusterId(1281);
const SS_IAS_WD: ClusterId = ClusterId(1282);
const GEN_TIME: ClusterId = ClusterId(10);
const GEN_OTA: ClusterId = ClusterId(25);

const fn new_register(endpoint: u8, app_prof_id: u16) -> Register {
    Register {
        endpoint,
        app_prof_id,
        app_device_id: 0x0005,
        app_dev_ver: 0,
        latency_req: LatencyRequirement::NoRequirement,
        in_clusters: Vec::new(),
        out_clusters: Vec::new(),
    }
}

pub fn default_endpoints() -> [Register; 14] {
    [
        new_register(1, 0x0104),
        new_register(2, 0x0101),
        // Required for https://github.com/Koenkk/zigbee-herdsman-converters/commit/d0fb06c2429171f327950484ea3dec80864637cc
        new_register(3, 0x0104),
        new_register(4, 0x0107),
        new_register(5, 0x0108),
        new_register(6, 0x0109),
        new_register(8, 0x0104),
        new_register(10, 0x0104),
        Register {
            endpoint: 11,
            app_prof_id: 0x0104,
            app_device_id: 0x0400,
            app_dev_ver: 0,
            latency_req: LatencyRequirement::NoRequirement,
            out_clusters: vec![SS_IAS_ZONE, SS_IAS_WD],
            // genTime required for https://github.com/Koenkk/zigbee2mqtt/issues/10816
            in_clusters: vec![SS_IAS_ACE, GEN_TIME],
        },
        // TERNCY: https://github.com/Koenkk/zigbee-herdsman/issues/82
        new_register(0x6e, 0x0104),
        new_register(12, 0xc05e),
        Register {
            endpoint: 13,
            app_prof_id: 0x0104,
            app_device_id: 0x0400,
            app_dev_ver: 0,
            latency_req: LatencyRequirement::NoRequirement,
            out_clusters: Vec::new(),
            // genTime required for https://github.com/Koenkk/zigbee2mqtt/issues/10816
            in_clusters: vec![GEN_OTA],
        },
        // Insta/Jung/Gira: OTA fallback EP (since it's buggy in firmware 10023202 when it tries to find a matching EP for
        // OTA - it queries for ZLL profile, but then contacts with HA profile)
        new_register(47, 0x0104),
        new_register(242, 0xa1e0),
    ]
}
