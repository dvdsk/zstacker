#[derive(Clone, Copy)]
pub enum InstallCodeFormat {
    InstallCodePlusCRC = 0x01,
    KeyDerivedFromInstallCode = 0x02,
}
