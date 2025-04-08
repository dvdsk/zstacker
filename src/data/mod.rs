use crate::api::{Command, CommandId, ParseByte};
use crate::constants::{
    CommissioningMode, InstallCodeFormat, LatencyReq, MtAFCommandId, MtAppConfigCommandId,
    MtCommandSubsystem, MtCommandType, MtSysCommandId, MtUtilCommandId, MtZdoCommandId,
    ResetRequestType, ScanChannels, TimeoutIndex,
};
use crate::wire::{encode_32, encode_64, encode_bytes, encode_short, encode_short_slice};

const MT_CMD_ID_MASK_SUB_SYS: u8 = 0x1F;
const MT_CMD_ID_MASK_TYPE: u8 = 0xE0;

#[derive(Clone, Copy)]
pub struct MtCommandId {
    cmd0: u8,
    cmd1: u8,
}

impl CommandId for MtCommandId {
    fn subsystem(&self) -> Option<MtCommandSubsystem> {
        MtCommandSubsystem::parse_byte(self.cmd0 & MT_CMD_ID_MASK_SUB_SYS)
    }

    fn cmd_type(&self) -> Option<MtCommandType> {
        MtCommandType::parse_byte(self.cmd0 & MT_CMD_ID_MASK_TYPE)
    }

    fn cmd_id(&self) -> u8 {
        self.cmd1
    }

    fn cmd0(&self) -> u8 {
        self.cmd0
    }

    fn cmd1(&self) -> u8 {
        self.cmd1
    }
}

impl MtCommandId {
    pub fn empty() -> Self {
        MtCommandId { cmd0: 0, cmd1: 0 }
    }

    pub fn new(subsystem: MtCommandSubsystem, cmd_type: MtCommandType, cmd_id: u8) -> Self {
        let cmd0 = (subsystem as u8) | (cmd_type as u8);

        MtCommandId { cmd0, cmd1: cmd_id }
    }
}

impl Default for MtCommandId {
    fn default() -> Self {
        MtCommandId::empty()
    }
}

pub struct MtCommand {
    data_len: u8,
    cmd: MtCommandId,
    data: [u8; 256],
}

impl Command for MtCommand {
    type CmdId = MtCommandId;

    fn cmd(&self) -> MtCommandId {
        self.cmd
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn len(&self) -> u8 {
        self.data_len
    }
}

impl MtCommand {
    pub fn empty() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::empty(),
            data: [0; 256],
        }
    }

    pub fn sys_reset_req(reset_type: ResetRequestType) -> Self {
        let mut data = [0; 256];
        data[0] = reset_type as u8;

        MtCommand {
            data_len: 0x01,
            cmd: MtCommandId::new(
                MtCommandSubsystem::SYSInterface,
                MtCommandType::SREQ,
                MtSysCommandId::SysResetReq as u8,
            ),
            data,
        }
    }

    pub fn sys_ping() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::new(
                MtCommandSubsystem::SYSInterface,
                MtCommandType::SREQ,
                MtSysCommandId::SysPing as u8,
            ),
            data: [0; 256],
        }
    }

    pub fn sys_version() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::new(
                MtCommandSubsystem::SYSInterface,
                MtCommandType::SREQ,
                MtSysCommandId::SysVersion as u8,
            ),
            data: [0; 256],
        }
    }

    pub fn sys_get_extaddr() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::new(
                MtCommandSubsystem::SYSInterface,
                MtCommandType::SREQ,
                MtSysCommandId::SysGetExtaddr as u8,
            ),
            data: [0; 256],
        }
    }

    pub fn sys_osal_start_timer(timer_id: u8, timeout: u16) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = timer_id;
        encode_short(timeout, &mut data, 1);

        MtCommand {
            data_len: 3,
            cmd: MtCommandId::new(
                MtCommandSubsystem::SYSInterface,
                MtCommandType::SREQ,
                MtSysCommandId::SysOsalStartTimer as u8,
            ),
            data,
        }
    }

    pub fn sys_osal_stop_timer(timer_id: u8) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = timer_id;

        MtCommand {
            data_len: 1,
            cmd: MtCommandId::new(
                MtCommandSubsystem::SYSInterface,
                MtCommandType::SREQ,
                MtSysCommandId::SysOsalStopTimer as u8,
            ),
            data,
        }
    }

    pub fn sys_random() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::new(
                MtCommandSubsystem::SYSInterface,
                MtCommandType::SREQ,
                MtSysCommandId::SysRandom as u8,
            ),
            data: [0; 256],
        }
    }

    pub fn sys_osal_nv_read(item_id: u16, offset: u8) -> Self {
        let mut data: [u8; 256] = [0; 256];
        encode_short(item_id, &mut data, 0);
        data[2] = offset;

        MtCommand {
            data_len: 3,
            cmd: MtCommandId::new(
                MtCommandSubsystem::SYSInterface,
                MtCommandType::SREQ,
                MtSysCommandId::SysOsalNvRead as u8,
            ),
            data,
        }
    }

    pub fn sys_osal_nv_write(item_id: u16, offset: u8, len: u8, value: &[u8]) -> Self {
        let mut data: [u8; 256] = [0; 256];
        encode_short(item_id, &mut data, 0);
        data[2] = offset;
        data[3] = len;
        // `value` must be up to 246 bytes in length
        encode_bytes(value, &mut data, 4);
        let data_len = (0x04_usize + value.len()) as u8;

        MtCommand {
            data_len,
            cmd: MtCommandId::new(
                MtCommandSubsystem::SYSInterface,
                MtCommandType::SREQ,
                MtSysCommandId::SysOsalNvWrite as u8,
            ),
            data,
        }
    }

    pub fn util_get_device_info() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::new(
                MtCommandSubsystem::UTILInterface,
                MtCommandType::SREQ,
                MtUtilCommandId::UtilGetDeviceInfo as u8,
            ),
            data: [0; 256],
        }
    }

    pub fn util_get_nv_info() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::new(
                MtCommandSubsystem::UTILInterface,
                MtCommandType::SREQ,
                MtUtilCommandId::UtilGetNvInfo as u8,
            ),
            data: [0; 256],
        }
    }

    pub fn util_time_alive() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::new(
                MtCommandSubsystem::UTILInterface,
                MtCommandType::SREQ,
                MtUtilCommandId::UtilTimeAlive as u8,
            ),
            data: [0; 256],
        }
    }

    pub fn util_callback_sub_cmd(subsystem_id: u16, should_enable: bool) -> Self {
        let mut data: [u8; 256] = [0; 256];
        encode_short(subsystem_id, &mut data, 0);
        data[2] = should_enable as u8;

        MtCommand {
            data_len: 0x03,
            cmd: MtCommandId::new(
                MtCommandSubsystem::UTILInterface,
                MtCommandType::SREQ,
                MtUtilCommandId::UtilCallbackSubCmd as u8,
            ),
            data,
        }
    }

    pub fn util_srng_gen() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::new(
                MtCommandSubsystem::UTILInterface,
                MtCommandType::SREQ,
                MtUtilCommandId::UtilSrngGen as u8,
            ),
            data: [0; 256],
        }
    }

    pub fn af_register(
        end_point: u8,
        app_prof_id: u16,
        app_device_id: u16,
        app_dev_ver: u8,
        latency_req: LatencyReq,
        app_num_in_clusters: u8,
        app_in_cluster_list: [u16; 16],
        app_num_out_clusters: u8,
        app_out_cluster_list: [u16; 16],
    ) -> Self {
        let data_len = 0x09 + (app_num_in_clusters * 2) + (app_num_out_clusters * 2);
        let mut data: [u8; 256] = [0; 256];
        data[0] = end_point;
        encode_short(app_prof_id, &mut data, 1);
        encode_short(app_device_id, &mut data, 3);
        data[5] = app_dev_ver;
        data[6] = latency_req as u8;
        data[7] = app_num_in_clusters;
        encode_short_slice(
            &app_in_cluster_list[..app_num_in_clusters as usize],
            &mut data,
            8,
        );
        let out_clusters_offset = 8 + (app_num_in_clusters * 2) as usize;
        data[out_clusters_offset] = app_num_out_clusters;
        encode_short_slice(
            &app_out_cluster_list[..app_num_out_clusters as usize],
            &mut data,
            out_clusters_offset + 1,
        );

        MtCommand {
            data_len,
            cmd: MtCommandId::new(
                MtCommandSubsystem::AFInterface,
                MtCommandType::SREQ,
                MtAFCommandId::AfRegister as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_set_nwk_frame_counter(frame_counter_value: u32) -> Self {
        let mut data: [u8; 256] = [0; 256];
        encode_32(frame_counter_value, &mut data, 0);

        MtCommand {
            data_len: 0x04,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfSetNwkFrameCounter as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_set_default_remote_enddevice_timeout(timeout_index: TimeoutIndex) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = timeout_index as u8;

        MtCommand {
            data_len: 0x01,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfSetDefaultRemoteEnddeviceTimeout as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_set_enddevicetimeout(timeout_index: TimeoutIndex) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = timeout_index as u8;

        MtCommand {
            data_len: 0x01,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfSetEnddevicetimeout as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_set_allowrejoin_tc_policy(allow_rejoin: bool) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = allow_rejoin as u8;

        MtCommand {
            data_len: 0x01,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfSetAllowrejoinTcPolicy as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_bdb_start_commissioning(commissioning_mode: CommissioningMode) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = commissioning_mode as u8;

        MtCommand {
            data_len: 0x01,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbStartCommissioning as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_bdb_set_channel(is_primary: bool, channel: ScanChannels) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = is_primary as u8;
        encode_32(channel as u32, &mut data, 1);

        MtCommand {
            data_len: 0x05,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbSetChannel as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_bdb_add_installcode(
        install_code_format: InstallCodeFormat,
        ieee_address: u64,
        install_code: [u8; 18],
    ) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = install_code_format as u8;
        encode_64(ieee_address, &mut data, 1);
        let (install_code_bytes, data_len) = match install_code_format {
            InstallCodeFormat::InstallCodePlusCRC => (&install_code[..], 0x1B),
            InstallCodeFormat::KeyDerivedFromInstallCode => (&install_code[..16], 0x19),
        };
        encode_bytes(install_code_bytes, &mut data, 9);

        MtCommand {
            data_len,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbAddInstallcode as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_bdb_set_tc_require_key_exchange(
        bdb_trust_center_require_key_exchange: bool,
    ) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = bdb_trust_center_require_key_exchange as u8;

        MtCommand {
            data_len: 0x01,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbSetTcRequireKeyExchange as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_bdb_set_joinusesinstallcodekey(bdb_join_uses_install_code_key: bool) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = bdb_join_uses_install_code_key as u8;

        MtCommand {
            data_len: 0x01,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbSetJoinusesinstallcodekey as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_bdb_set_active_default_centralized_key(
        use_global: bool,
        install_code: [u8; 18],
    ) -> Self {
        let mut data: [u8; 256] = [0; 256];
        data[0] = use_global as u8;
        encode_bytes(&install_code, &mut data, 1);

        MtCommand {
            data_len: 0x13,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbSetActiveDefaultCentralizedKey as u8,
            ),
            data,
        }
    }

    pub fn app_cnf_bdb_zed_attempt_recover_nwk() -> Self {
        MtCommand {
            data_len: 0x00,
            cmd: MtCommandId::new(
                MtCommandSubsystem::APPConfig,
                MtCommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbZedAttemptRecoverNwk as u8,
            ),
            data: [0; 256],
        }
    }

    pub fn zdo_nwk_discovery_req(scan_channels: ScanChannels, scan_duration: u8) -> Self {
        let mut data: [u8; 256] = [0; 256];
        encode_32(scan_channels as u32, &mut data, 0);
        data[4] = scan_duration;

        MtCommand {
            data_len: 0x05,
            cmd: MtCommandId::new(
                MtCommandSubsystem::ZDOInterface,
                MtCommandType::SREQ,
                MtZdoCommandId::ZdoNwkDiscoveryReq as u8,
            ),
            data,
        }
    }
    // TODO - implement ZDO_NODE_DESC_REQ to get node capabilities
    // TODO - implement ZDO_POWER_DESC_REQ to get node power status
    // TODO - implement ZDO_SIMPLE_DESC_REQ to get node simple descriptor
    // TODO - implement ZDO_COMPLEX_DESC_REQ to get node complex descriptor
    // TODO - implement ZDO_USER_DESC_REQ to get node user descriptor
    // TODO - implement ZDO_ACTIVE_EP_REQ to get node active endpoints list
    // TODO - implement ZDO_END_DEVICE_ANNCE
}

impl Default for MtCommand {
    fn default() -> Self {
        MtCommand::empty()
    }
}
