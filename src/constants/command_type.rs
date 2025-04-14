use crate::api::ParseByte;

#[derive(Clone, Copy)]
pub enum CommandType {
    POLL = 0x00,
    SREQ = 0x20,
    AREQ = 0x40,
    SRSP = 0x60,
}

// TODO - use derive(ParseByte)
impl ParseByte<Self> for CommandType {
    fn parse_byte(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(CommandType::POLL),
            0x20 => Some(CommandType::SREQ),
            0x40 => Some(CommandType::AREQ),
            0x60 => Some(CommandType::SRSP),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt_command_type() {
        assert_eq!(CommandType::POLL as u8, 0x00);
        assert_eq!(CommandType::SREQ as u8, 0x20);
        assert_eq!(CommandType::AREQ as u8, 0x40);
        assert_eq!(CommandType::SRSP as u8, 0x60);
    }
}
