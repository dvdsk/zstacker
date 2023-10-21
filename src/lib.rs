pub const SOF: u8 = 0xFE;
pub const MT_CMD_ID_SUB_SYS: u8 = 0x1F;
pub const MT_CMD_ID_TYPE: u8 = 0xE0;
pub const MT_CMD_ID_TYPE_SHIFT: usize = 5;
pub const MT_CMD_BASE_CAP: usize = 5; // SOF(1) + MTCMD(3) + FCS(1)

pub enum MtCommandType {
    POLL = 0x00,
    SREQ = 0x20,
    AREQ = 0x40,
    SRSP = 0x60,
}

pub enum Subsystem {
    Reserved = 0x00,
    SYSInterface = 0x01,
    MACInterface = 0x02,
    NWKInterface = 0x03,
    AFInterface = 0x04,
    ZDOInterface = 0x05,
    SAPIInterface = 0x06,
    UTILInterface = 0x07,
    DEBUGInterface = 0x08,
    APPInterface = 0x09,
    APPConfig = 0x0F,
    GreenPower = 0x15,
}

pub enum MtCommandSubsystem {}
pub struct MtCommandId {
    cmd0: u8,
    cmd1: u8,
}

pub trait CommandId {
    fn subsystem(&self) -> u8;
    fn cmd_type(&self) -> u8;
    fn cmd_id(&self) -> u8;
}

impl CommandId for MtCommandId {
    fn subsystem(&self) -> u8 {
        self.cmd0 & MT_CMD_ID_SUB_SYS
    }

    fn cmd_type(&self) -> u8 {
        (self.cmd0 & MT_CMD_ID_TYPE) >> MT_CMD_ID_TYPE_SHIFT
    }

    fn cmd_id(&self) -> u8 {
        self.cmd1
    }
}

impl MtCommandId {
    fn empty() -> Self {
        MtCommandId { cmd0: 0, cmd1: 0 }
    }
}

impl Default for MtCommandId {
    fn default() -> Self {
        MtCommandId::empty()
    }
}

// TODO implement for MtCommand
pub trait Command {
    fn len(&self) -> u8;
    fn cmd(&self) -> [u8; 2];
    fn data(&self) -> [u8];
}

pub struct MtCommand {
    data_len: u8,
    cmd: MtCommandId,
    data: [u8; 256],
}

impl MtCommand {
    fn empty() -> Self {
        MtCommand {
            data_len: 0,
            cmd: MtCommandId::empty(),
            data: [0; 256],
        }
    }
}

impl Default for MtCommand {
    fn default() -> Self {
        MtCommand::empty()
    }
}

pub struct GeneralSerialPacket {
    mt_cmd: MtCommand,
    fcs: u8,
}

impl GeneralSerialPacket {
    fn from_cmd(mt_cmd: MtCommand) -> Self {
        let cmd_id = &mt_cmd.cmd;

        let fcs = cmd_id.cmd0
            ^ cmd_id.cmd1
            ^ mt_cmd.data[0..(mt_cmd.data_len as usize)]
                .iter()
                .fold(0, |x, y| x ^ y);

        GeneralSerialPacket {
            mt_cmd: mt_cmd,
            fcs: fcs,
        }
    }

    fn to_frame(&self) -> Vec<u8> {
        //TODO change the len()
        let mut frame = Vec::with_capacity(MT_CMD_BASE_CAP + (self.mt_cmd.data_len as usize));
        frame.push(SOF);
        frame.push(self.mt_cmd.cmd.cmd0);
        frame.push(self.mt_cmd.cmd.cmd1);
        self.mt_cmd.data.iter().for_each(|d| frame.push(*d));
        frame.push(self.fcs);

        frame
    }
}
