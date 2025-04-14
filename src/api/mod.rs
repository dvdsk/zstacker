// TODO - write a Derive macro
pub trait ParseByte<T> {
    fn parse_byte(value: u8) -> Option<T>;
}
