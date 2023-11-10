#[derive(Clone, Copy)]
pub enum CommissioningMode {
    Initialization = 0x00,
    TouchLink = 0x01,
    NetworkSteering = 0x02,
    NetworkFormation = 0x04,
    FindingAndBinding = 0x08,
}
