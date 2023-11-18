#[derive(Clone, Copy)]
pub enum SubSystemId {
    MtSys = 0x0100,
    MtMac = 0x0200,
    MtNwk = 0x0300,
    MtAf = 0x0400,
    MtZdo = 0x0500,
    MtSapi = 0x0600,
    MtUtil = 0x0700,
    MtDebug = 0x0800,
    MtApp = 0x0900,
    AllSubsystem = 0xFFFF,
}

impl SubSystemId {
    pub fn of(sub_systems: &[SubSystemId]) -> u16 {
        let mut systems = 0x0000;
        for sys in sub_systems {
            systems = systems | *sys as u16;
        }

        systems
    }
}
