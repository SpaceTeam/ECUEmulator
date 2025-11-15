use zerocopy_derive::{FromBytes, Immutable, IntoBytes};

pub mod common_command_discriminants {
    pub const REQ_RESET_SETTINGS: isize = 0;
    pub const RES_RESET_SETTINGS: isize = 1;
    pub const REQ_STATUS: isize = 2;
    pub const RES_STATUS: isize = 3;
    pub const REQ_SET_VARIABLE: isize = 4;
    pub const RES_SET_VARIABLE: isize = 5;
    pub const REQ_GET_VARIABLE: isize = 6;
    pub const RES_GET_VARIABLE: isize = 7;
    pub const COMMON_TOTAL_CMDS: isize = 8;
}

#[repr(isize)]
pub enum CommonCommands {
    CommonReqResetSettings = common_command_discriminants::REQ_RESET_SETTINGS, // NO payload
    CommonResResetSettings = common_command_discriminants::RES_RESET_SETTINGS, // NO payload
    CommonReqStatus = common_command_discriminants::REQ_STATUS,                // NO payload
    CommonResStatus = common_command_discriminants::RES_STATUS, // TODO: some status msg
    CommonReqSetVariable {
        payload: SetMsgPayload,
    } = common_command_discriminants::REQ_SET_VARIABLE, // SetMsg_t
    CommonResSetVariable {
        payload: SetMsgPayload,
    } = common_command_discriminants::RES_SET_VARIABLE, // SetMsg_t
    CommonReqGetVariable {
        payload: GetMsgPayload,
    } = common_command_discriminants::REQ_GET_VARIABLE, // GetMsg_t
    CommonResGetVariable {
        payload: SetMsgPayload,
    } = common_command_discriminants::RES_GET_VARIABLE, // SetMsg_t

    CommonTotalCmds = common_command_discriminants::COMMON_TOTAL_CMDS,
}
impl From<CommonCommands> for u8 {
    fn from(value: CommonCommands) -> Self {
        match value {
            CommonCommands::CommonReqResetSettings => 0,
            CommonCommands::CommonResResetSettings => 1,
            CommonCommands::CommonReqStatus => 2,
            CommonCommands::CommonResStatus => 3,
            CommonCommands::CommonReqSetVariable { .. } => 4,
            CommonCommands::CommonResSetVariable { .. } => 5,
            CommonCommands::CommonReqGetVariable { .. } => 6,
            CommonCommands::CommonResGetVariable { .. } => 7,
            CommonCommands::CommonTotalCmds => 8,
        }
    }
}
#[derive(Debug)]
#[derive(FromBytes,IntoBytes,Immutable)]
#[repr(C, packed)]
pub struct SetMsgPayload {
    pub variable_id: u8,
    pub value: u32,
}
#[derive(Debug)]
#[derive(FromBytes,IntoBytes,Immutable)]
#[repr(C, packed)]
pub struct GetMsgPayload {
    pub variable_id: u8,
}
