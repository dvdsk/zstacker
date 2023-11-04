use crate::api::{Command, CommandId, ParseByte};
use crate::constants::{MtCommandSubsystem, MtCommandType, MtSysCommandId, MtUtilCommandId};

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

    pub fn new(
        subsystem: MtCommandSubsystem,
        cmd_type: MtCommandType,
        cmd_id: u8,
    ) -> Self {
        let cmd0 = (subsystem as u8) | (cmd_type as u8);

        MtCommandId {
            cmd0,
            cmd1: cmd_id,
        }
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
        data[1] = (timeout & 0x00FF) as u8;
        data[2] = ((timeout & 0xFF00) >> 8) as u8;

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

    pub fn util_get_device_info() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::new(
                MtCommandSubsystem::UTILInterface,
                MtCommandType::SREQ,
                MtUtilCommandId::UTIL_GET_DEVICE_INFO as u8,
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
                MtUtilCommandId::UTIL_GET_NV_INFO as u8,
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
                MtUtilCommandId::UTIL_TIME_ALIVE as u8,
            ),
            data: [0; 256],
        }
    }

    pub fn util_srng_gen() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::new(
                MtCommandSubsystem::UTILInterface,
                MtCommandType::SREQ,
                MtUtilCommandId::UTIL_SRNG_GEN as u8,
            ),
            data: [0; 256],
        }
    }
}

impl Default for MtCommand {
    fn default() -> Self {
        MtCommand::empty()
    }
}
