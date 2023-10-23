use crate::api::ParseByte;

#[derive(Clone, Copy)]
pub enum MtCommandType {
    POLL = 0x00,
    SREQ = 0x20,
    AREQ = 0x40,
    SRSP = 0x60,
}

// TODO - use derive(ParseByte)
impl ParseByte<Self> for MtCommandType {
    fn parse_byte(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(MtCommandType::POLL),
            0x20 => Some(MtCommandType::SREQ),
            0x40 => Some(MtCommandType::AREQ),
            0x60 => Some(MtCommandType::SRSP),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt_command_type() {
        assert_eq!(MtCommandType::POLL as u8, 0x00);
        assert_eq!(MtCommandType::SREQ as u8, 0x20);
        assert_eq!(MtCommandType::AREQ as u8, 0x40);
        assert_eq!(MtCommandType::SRSP as u8, 0x60);
    }
}
