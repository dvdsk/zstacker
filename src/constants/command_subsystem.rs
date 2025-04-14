use crate::api::ParseByte;

#[derive(Clone, Copy)]
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

// TODO - use derive(ParseByte)
impl ParseByte<Self> for Subsystem {
    fn parse_byte(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Subsystem::Reserved),
            0x01 => Some(Subsystem::SYSInterface),
            0x02 => Some(Subsystem::MACInterface),
            0x03 => Some(Subsystem::NWKInterface),
            0x04 => Some(Subsystem::AFInterface),
            0x05 => Some(Subsystem::ZDOInterface),
            0x06 => Some(Subsystem::SAPIInterface),
            0x07 => Some(Subsystem::UTILInterface),
            0x08 => Some(Subsystem::DEBUGInterface),
            0x09 => Some(Subsystem::APPInterface),
            0x0F => Some(Subsystem::APPConfig),
            0x15 => Some(Subsystem::GreenPower),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt_command_subystem() {
        assert_eq!(Subsystem::Reserved as u8, 0x00);
        assert_eq!(Subsystem::SYSInterface as u8, 0x01);
        assert_eq!(Subsystem::MACInterface as u8, 0x02);
        assert_eq!(Subsystem::NWKInterface as u8, 0x03);
        assert_eq!(Subsystem::AFInterface as u8, 0x04);
        assert_eq!(Subsystem::ZDOInterface as u8, 0x05);
        assert_eq!(Subsystem::SAPIInterface as u8, 0x06);
        assert_eq!(Subsystem::UTILInterface as u8, 0x07);
        assert_eq!(Subsystem::DEBUGInterface as u8, 0x08);
        assert_eq!(Subsystem::APPInterface as u8, 0x09);
        assert_eq!(Subsystem::APPConfig as u8, 0x0F);
        assert_eq!(Subsystem::GreenPower as u8, 0x15);
    }
}
