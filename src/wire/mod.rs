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

        GeneralSerialPacket { mt_cmd, fcs }
    }

    pub fn to_frame(&self) -> Vec<u8> {
        let d_len = self.mt_cmd.len() as usize;
        let mut frame = Vec::with_capacity(MT_CMD_BASE_CAP + d_len);
        let cmd_id = self.mt_cmd.cmd();

        frame.push(SOF);
        frame.push(self.mt_cmd.len());
        frame.push(cmd_id.cmd0());
        frame.push(cmd_id.cmd1());
        self.mt_cmd.data()[0..d_len]
            .iter()
            .for_each(|d| frame.push(*d));
        frame.push(self.fcs);

        frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::MtCommand;

    #[test]
    fn test_serialization_sys_ping() {
        let mt_cmd = MtCommand::sys_ping();
        let packet = GeneralSerialPacket::from_cmd(mt_cmd);
        let expected: Vec<u8> = vec![0xFE, 0x00, 0x21, 0x01, 0x20];

        assert_eq!(packet.to_frame(), expected);
    }
}