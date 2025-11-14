use zerocopy_derive::{FromBytes, Immutable, IntoBytes};

#[repr(isize)]
pub enum CommonCommands {
    CommonReqResetSettings = 0,                          // NO payload
    CommonResResetSettings = 1,                          // NO payload
    CommonReqStatus = 2,                                 // NO payload
    CommonResStatus = 3,                                 // TODO: some status msg
    CommonReqSetVariable { payload: SetMsgPayload } = 4, // SetMsg_t
    CommonResSetVariable { payload: SetMsgPayload } = 5, // SetMsg_t
    CommonReqGetVariable { payload: GetMsgPayload } = 6, // GetMsg_t
    CommonResGetVariable { payload: SetMsgPayload } = 7, // SetMsg_t

    CommonTotalCmds,
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
