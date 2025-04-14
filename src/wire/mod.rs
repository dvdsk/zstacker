use crate::constants::START_OF_FRAME;
use crate::data::Command;
const MT_CMD_BASE_CAP: usize = 5; // SOF(1) + MTCMD(3) + FCS(1)

pub struct Packet {
    cmd: Command,
    fcs: u8,
}

impl Packet {
    pub fn from_cmd(cmd: Command) -> Self {
        let cmd_id = cmd.cmd();
        let d_len = cmd.len() as usize;

        let fcs = cmd.data()[0..d_len]
            .iter()
            .fold(cmd.len() ^ cmd_id.cmd0() ^ cmd_id.cmd1(), |x, y| x ^ y);

        Packet { cmd, fcs }
    }

    pub fn to_frame(&self) -> Vec<u8> {
        let d_len = self.cmd.len() as usize;
        let mut frame = Vec::with_capacity(MT_CMD_BASE_CAP + d_len);
        let cmd_id = self.cmd.cmd();

        frame.push(START_OF_FRAME);
        frame.push(self.cmd.len());
        frame.push(cmd_id.cmd0());
        frame.push(cmd_id.cmd1());
        self.cmd.data()[0..d_len]
            .iter()
            .for_each(|d| frame.push(*d));
        frame.push(self.fcs);

        frame
    }
}

/// THIS IS short.to_le_bytes() shifted in some buffer
pub fn encode_short(short: u16, buf: &mut [u8], offset: usize) {
    buf[offset] = (short & 0xFF) as u8;
    buf[offset + 1] = ((short >> 8) & 0xFF) as u8;
}

/// THIS IS num.to_le_bytes() shifted in some buffer
pub fn encode_32(num: u32, buf: &mut [u8], offset: usize) {
    buf[offset] = (num & 0xFF) as u8;
    buf[offset + 1] = ((num >> 8) & 0xFF) as u8;
    buf[offset + 2] = ((num >> 16) & 0xFF) as u8;
    buf[offset + 3] = ((num >> 24) & 0xFF) as u8;
}

/// THIS IS num.to_le_bytes() shifted in some buffer
pub fn encode_64(num: u64, buf: &mut [u8], offset: usize) {
    encode_32((num & 0xFF_FF_FF_FF) as u32, buf, offset);
    encode_32(((num >> 32) & 0xFF_FF_FF_FF) as u32, buf, offset + 4);
}

pub fn encode_short_slice(shorts: &[u16], buf: &mut [u8], offset: usize) {
    let buf_len = buf.len();

    if offset >= buf_len || offset + (shorts.len() * 2) >= buf_len {
        panic!("Offset and length exceed the buffer size");
    }

    let mut i = offset;
    for short in shorts {
        encode_short(*short, buf, i);
        i += 2;
    }
}

// THIS IS copy_from_slice starting at some offset
pub fn encode_bytes(bytes: &[u8], buf: &mut [u8], offset: usize) {
    let buf_len = buf.len();

    if offset >= buf_len || offset + bytes.len() >= buf_len {
        panic!("Offset and length exceed the buffer size");
    }

    buf[offset..offset + bytes.len()].copy_from_slice(bytes);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Command;

    #[test]
    fn test_serialization_sys_ping() {
        let mt_cmd = Command::sys_ping();
        let packet = Packet::from_cmd(mt_cmd);
        let expected: Vec<u8> = vec![0xFE, 0x00, 0x21, 0x01, 0x20];

        assert_eq!(packet.to_frame(), expected);
    }
}
