pub const SOF: u8 = 0xFE;

pub enum MtCommandType {
    POLL = 0x00,
    SREQ = 0x20,
    AREQ = 0x40,
    SRSP = 0x60,
}
impl MtCommandType {
    pub fn parse_byte(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(MtCommandType::POLL),
            0x20 => Some(MtCommandType::SREQ),
            0x40 => Some(MtCommandType::AREQ),
            0x60 => Some(MtCommandType::SRSP),
            _ => None,
        }
    }
}

pub enum MtCommandSubsystem {
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

impl MtCommandSubsystem {
    pub fn parse_byte(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(MtCommandSubsystem::Reserved),
            0x01 => Some(MtCommandSubsystem::SYSInterface),
            0x02 => Some(MtCommandSubsystem::MACInterface),
            0x03 => Some(MtCommandSubsystem::NWKInterface),
            0x04 => Some(MtCommandSubsystem::AFInterface),
            0x05 => Some(MtCommandSubsystem::ZDOInterface),
            0x06 => Some(MtCommandSubsystem::SAPIInterface),
            0x07 => Some(MtCommandSubsystem::UTILInterface),
            0x08 => Some(MtCommandSubsystem::DEBUGInterface),
            0x09 => Some(MtCommandSubsystem::APPInterface),
            0x0F => Some(MtCommandSubsystem::APPConfig),
            0x15 => Some(MtCommandSubsystem::GreenPower),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt_command_subystem() {
        assert_eq!(MtCommandSubsystem::Reserved as u8, 0x00);
        assert_eq!(MtCommandSubsystem::SYSInterface as u8, 0x01);
        assert_eq!(MtCommandSubsystem::MACInterface as u8, 0x02);
        assert_eq!(MtCommandSubsystem::NWKInterface as u8, 0x03);
        assert_eq!(MtCommandSubsystem::AFInterface as u8, 0x04);
        assert_eq!(MtCommandSubsystem::ZDOInterface as u8, 0x05);
        assert_eq!(MtCommandSubsystem::SAPIInterface as u8, 0x06);
        assert_eq!(MtCommandSubsystem::UTILInterface as u8, 0x07);
        assert_eq!(MtCommandSubsystem::DEBUGInterface as u8, 0x08);
        assert_eq!(MtCommandSubsystem::APPInterface as u8, 0x09);
        assert_eq!(MtCommandSubsystem::APPConfig as u8, 0x0F);
        assert_eq!(MtCommandSubsystem::GreenPower as u8, 0x15);
    }

    #[test]
    fn test_mt_command_type() {
        assert_eq!(MtCommandType::POLL as u8, 0x00);
        assert_eq!(MtCommandType::SREQ as u8, 0x20);
        assert_eq!(MtCommandType::AREQ as u8, 0x40);
        assert_eq!(MtCommandType::SRSP as u8, 0x60);
    }
}
