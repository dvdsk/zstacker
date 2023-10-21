use crate::api::{Command, CommandId};
use crate::constants::SOF;
const MT_CMD_BASE_CAP: usize = 5; // SOF(1) + MTCMD(3) + FCS(1)

pub struct GeneralSerialPacket<T: Command> {
    mt_cmd: T,
    fcs: u8,
}

impl<T: Command> GeneralSerialPacket<T> {
    pub fn from_cmd(mt_cmd: T) -> Self {
        let cmd_id = mt_cmd.cmd();
        let d_len = mt_cmd.len() as usize;

        let fcs = mt_cmd.data()[0..d_len]
            .iter()
            .fold(cmd_id.cmd0() ^ cmd_id.cmd1(), |x, y| x ^ y);

        GeneralSerialPacket { mt_cmd, fcs: fcs }
    }

    pub fn to_frame(&self) -> Vec<u8> {
        let mut frame = Vec::with_capacity(MT_CMD_BASE_CAP + (self.mt_cmd.len() as usize));
        let cmd_id = self.mt_cmd.cmd();
        let d_len = self.mt_cmd.len() as usize;

        frame.push(SOF);
        frame.push(cmd_id.cmd0());
        frame.push(cmd_id.cmd1());
        self.mt_cmd.data()[0..d_len]
            .iter()
            .for_each(|d| frame.push(*d));
        frame.push(self.fcs);

        frame
    }
}
