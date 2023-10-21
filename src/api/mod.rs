use crate::constants::{MtCommandSubsystem, MtCommandType};

pub trait CommandId {
    fn subsystem(&self) -> Option<MtCommandSubsystem>;
    fn cmd_type(&self) -> Option<MtCommandType>;
    fn cmd_id(&self) -> u8;
    fn cmd0(&self) -> u8;
    fn cmd1(&self) -> u8;
}

pub trait Command {
    type CmdId: CommandId;

    fn len(&self) -> u8;
    fn cmd(&self) -> Self::CmdId;
    fn data(&self) -> &[u8];
}
