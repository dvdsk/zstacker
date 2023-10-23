use crate::constants::{MtCommandSubsystem, MtCommandType};

pub trait CommandId {
    fn subsystem(&self) -> Option<MtCommandSubsystem>;
    fn cmd_type(&self) -> Option<MtCommandType>;
    fn cmd_id(&self) -> u8;
    fn cmd0(&self) -> u8;
    fn cmd1(&self) -> u8;
}

#[allow(clippy::len_without_is_empty)]
pub trait Command {
    type CmdId: CommandId;

    fn len(&self) -> u8;
    fn cmd(&self) -> Self::CmdId;
    fn data(&self) -> &[u8];
}

// TODO - write a Derive macro 
pub trait ParseByte<T> {
    fn parse_byte(value: u8) -> Option<T>;
}