use crate::api::{Command, CommandId, ParseByte};
use crate::constants::{MtCommandSubsystem, MtCommandType, MtSysCommandId};

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
        cmd_id: MtSysCommandId,
    ) -> Self {
        let cmd0 = (subsystem as u8) | (cmd_type as u8);

        MtCommandId {
            cmd0,
            cmd1: cmd_id as u8,
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
                MtSysCommandId::SysPing,
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
                MtSysCommandId::SysVersion,
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
