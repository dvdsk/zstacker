use std::iter;

use crate::api::ParseByte;
use crate::constants::{
    AFCommandId, CommandType, CommissioningMode, InstallCodeFormat, LatencyReq,
    MtAppConfigCommandId, MtSysCommandId, UtilCommandId, MtZdoCommandId, ResetRequestType,
    ScanChannels, Subsystem, TimeoutIndex,
};

const MT_CMD_ID_MASK_SUB_SYS: u8 = 0x1F;
const MT_CMD_ID_MASK_TYPE: u8 = 0xE0;

#[derive(Debug, Clone, Copy)]
pub struct CommandId {
    cmd0: u8,
    cmd1: u8,
}

impl CommandId {
    pub fn subsystem(&self) -> Option<Subsystem> {
        Subsystem::parse_byte(self.cmd0 & MT_CMD_ID_MASK_SUB_SYS)
    }

    pub fn cmd_type(&self) -> Option<CommandType> {
        CommandType::parse_byte(self.cmd0 & MT_CMD_ID_MASK_TYPE)
    }

    pub fn cmd_id(&self) -> u8 {
        self.cmd1
    }

    pub fn cmd0(&self) -> u8 {
        self.cmd0
    }

    pub fn cmd1(&self) -> u8 {
        self.cmd1
    }
}

impl CommandId {
    pub fn empty() -> Self {
        CommandId { cmd0: 0, cmd1: 0 }
    }

    pub fn new(subsystem: Subsystem, cmd_type: CommandType, cmd_id: u8) -> Self {
        let cmd0 = (subsystem as u8) | (cmd_type as u8);

        CommandId { cmd0, cmd1: cmd_id }
    }
}

impl Default for CommandId {
    fn default() -> Self {
        CommandId::empty()
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    cmd: CommandId,
    data: Vec<u8>,
}

impl Command {
    pub fn cmd(&self) -> CommandId {
        self.cmd
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> u8 {
        self.data
            .len()
            .try_into()
            .expect("data should be shorter then 256 bytes")
    }

    pub fn reply_len(&self) -> usize {
        todo!()
    }
}

impl Command {
    pub fn empty() -> Self {
        Command {
            cmd: CommandId::empty(),
            data: Vec::new(),
        }
    }

    pub fn sys_reset_req(reset_type: ResetRequestType) -> Self {
        let mut data = Vec::new();
        data.push(reset_type as u8);

        Command {
            cmd: CommandId::new(
                Subsystem::SYSInterface,
                CommandType::SREQ,
                MtSysCommandId::SysResetReq as u8,
            ),
            data,
        }
    }

    pub fn sys_ping() -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::SYSInterface,
                CommandType::SREQ,
                MtSysCommandId::SysPing as u8,
            ),
            data: Vec::new(),
        }
    }

    pub fn sys_version() -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::SYSInterface,
                CommandType::SREQ,
                MtSysCommandId::SysVersion as u8,
            ),
            data: Vec::new(),
        }
    }

    pub fn sys_get_extaddr() -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::SYSInterface,
                CommandType::SREQ,
                MtSysCommandId::SysGetExtaddr as u8,
            ),
            data: Vec::new(),
        }
    }

    pub fn sys_osal_start_timer(timer_id: u8, timeout: u16) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::SYSInterface,
                CommandType::SREQ,
                MtSysCommandId::SysOsalStartTimer as u8,
            ),
            data: iter::once(timer_id)
                .chain(timeout.to_le_bytes().into_iter())
                .collect(),
        }
    }

    pub fn sys_osal_stop_timer(timer_id: u8) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::SYSInterface,
                CommandType::SREQ,
                MtSysCommandId::SysOsalStopTimer as u8,
            ),
            data: vec![timer_id],
        }
    }

    pub fn sys_random() -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::SYSInterface,
                CommandType::SREQ,
                MtSysCommandId::SysRandom as u8,
            ),
            data: Vec::new(),
        }
    }

    pub fn sys_osal_nv_read(item_id: u16, offset: u8) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::SYSInterface,
                CommandType::SREQ,
                MtSysCommandId::SysOsalNvRead as u8,
            ),
            data: item_id
                .to_le_bytes()
                .into_iter()
                .chain(std::iter::once(offset))
                .collect(),
        }
    }

    pub fn sys_osal_nv_write(item_id: u16, offset: u8, len: u8, value: &[u8]) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::SYSInterface,
                CommandType::SREQ,
                MtSysCommandId::SysOsalNvWrite as u8,
            ),
            data: item_id
                .to_le_bytes()
                .into_iter()
                .chain(iter::once(offset))
                .chain(iter::once(len))
                .chain(value.iter().copied())
                .collect(),
        }
    }

    pub fn get_device_info() -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::UTILInterface,
                CommandType::SREQ,
                UtilCommandId::GetDeviceInfo as u8,
            ),
            data: Vec::new(),
        }
    }

    pub fn util_get_nv_info() -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::UTILInterface,
                CommandType::SREQ,
                UtilCommandId::GetNvInfo as u8,
            ),
            data: Vec::new(),
        }
    }

    pub fn util_time_alive() -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::UTILInterface,
                CommandType::SREQ,
                UtilCommandId::TimeAlive as u8,
            ),
            data: Vec::new(),
        }
    }

    pub fn util_callback_sub_cmd(subsystem_id: u16, should_enable: bool) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::UTILInterface,
                CommandType::SREQ,
                UtilCommandId::CallbackSubCmd as u8,
            ),
            data: subsystem_id
                .to_le_bytes()
                .into_iter()
                .chain([should_enable as u8])
                .collect(),
        }
    }

    pub fn util_srng_gen() -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::UTILInterface,
                CommandType::SREQ,
                UtilCommandId::SrngGen as u8,
            ),
            data: Vec::new(),
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
        Command {
            cmd: CommandId::new(
                Subsystem::AFInterface,
                CommandType::SREQ,
                AFCommandId::AfRegister as u8,
            ),
            data: iter::once(end_point)
                .chain(app_prof_id.to_le_bytes())
                .chain(app_device_id.to_le_bytes())
                .chain([app_dev_ver, latency_req as u8, app_num_in_clusters])
                .chain(
                    app_in_cluster_list
                        .into_iter()
                        .take(app_num_in_clusters as usize)
                        .flat_map(u16::to_le_bytes),
                )
                .chain([app_num_out_clusters])
                .chain(
                    app_out_cluster_list
                        .into_iter()
                        .take(app_num_out_clusters as usize)
                        .flat_map(u16::to_le_bytes),
                )
                .collect(),
        }
    }

    pub fn app_cnf_set_nwk_frame_counter(frame_counter_value: u32) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfSetNwkFrameCounter as u8,
            ),
            data: frame_counter_value.to_le_bytes().to_vec(),
        }
    }

    pub fn app_cnf_set_default_remote_enddevice_timeout(timeout_index: TimeoutIndex) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfSetDefaultRemoteEnddeviceTimeout as u8,
            ),
            data: vec![timeout_index as u8],
        }
    }

    pub fn app_cnf_set_enddevicetimeout(timeout_index: TimeoutIndex) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfSetEnddevicetimeout as u8,
            ),
            data: vec![timeout_index as u8],
        }
    }

    pub fn app_cnf_set_allowrejoin_tc_policy(allow_rejoin: bool) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfSetAllowrejoinTcPolicy as u8,
            ),
            data: vec![allow_rejoin as u8],
        }
    }

    pub fn app_cnf_bdb_start_commissioning(commissioning_mode: CommissioningMode) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbStartCommissioning as u8,
            ),
            data: vec![commissioning_mode as u8],
        }
    }

    pub fn app_cnf_bdb_set_channel(is_primary: bool, channel: ScanChannels) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbSetChannel as u8,
            ),
            data: iter::once(is_primary as u8)
                .chain((channel as u32).to_le_bytes())
                .collect(),
        }
    }

    pub fn app_cnf_bdb_add_installcode(
        install_code_format: InstallCodeFormat,
        ieee_address: u64,
        install_code: [u8; 18],
    ) -> Self {
        let install_code = match install_code_format {
            InstallCodeFormat::InstallCodePlusCRC => install_code.into_iter().take(usize::MAX),
            InstallCodeFormat::KeyDerivedFromInstallCode => install_code.into_iter().take(16),
        };

        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbAddInstallcode as u8,
            ),
            data: iter::once(install_code_format as u8)
                .chain(ieee_address.to_le_bytes())
                .chain(install_code)
                .collect(),
        }
    }

    pub fn app_cnf_bdb_set_tc_require_key_exchange(
        bdb_trust_center_require_key_exchange: bool,
    ) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbSetTcRequireKeyExchange as u8,
            ),
            data: vec![bdb_trust_center_require_key_exchange as u8],
        }
    }

    pub fn app_cnf_bdb_set_joinusesinstallcodekey(bdb_join_uses_install_code_key: bool) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbSetJoinusesinstallcodekey as u8,
            ),
            data: vec![bdb_join_uses_install_code_key as u8],
        }
    }

    pub fn app_cnf_bdb_set_active_default_centralized_key(
        use_global: bool,
        install_code: [u8; 18],
    ) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbSetActiveDefaultCentralizedKey as u8,
            ),
            data: iter::once(use_global as u8).chain(install_code).collect(),
        }
    }

    pub fn app_cnf_bdb_zed_attempt_recover_nwk() -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::APPConfig,
                CommandType::SREQ,
                MtAppConfigCommandId::AppCnfBdbZedAttemptRecoverNwk as u8,
            ),
            data: Vec::new(),
        }
    }

    pub fn zdo_nwk_discovery_req(scan_channels: ScanChannels, scan_duration: u8) -> Self {
        Command {
            cmd: CommandId::new(
                Subsystem::ZDOInterface,
                CommandType::SREQ,
                MtZdoCommandId::ZdoNwkDiscoveryReq as u8,
            ),
            data: (scan_channels as u32)
                .to_le_bytes()
                .into_iter()
                .chain(iter::once(scan_duration))
                .collect(),
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

impl Default for Command {
    fn default() -> Self {
        Command::empty()
    }
}
