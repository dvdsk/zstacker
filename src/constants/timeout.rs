#[derive(Clone, Copy)]
pub enum TimeoutIndex {
    Seconds10 = 0x00,
    Minutes2 = 0x01,
    Minutes4 = 0x02,
    Minutes8 = 0x03,
    Minutes16 = 0x04,
    Minutes32 = 0x05,
    Minutes64 = 0x06,
    Minutes128 = 0x07,
    Minutes256 = 0x08,
    Minutes512 = 0x09,
    Minutes1024 = 0x0A,
    Minutes2048 = 0x0B,
    Minutes4096 = 0x0C,
    Minutes8192 = 0x0D,
    Minutes16384 = 0x0E,
}
