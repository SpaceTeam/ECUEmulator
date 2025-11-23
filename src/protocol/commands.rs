use ecu_emulator_macros_derive::EnumDiscriminate;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, EnumDiscriminate)]
#[repr(u8)]
pub enum CommonCommands {
    CommonReqResetSettings = CommonCommandsDiscriminant::CommonReqResetSettings.discriminant(), // NO payload
    CommonResResetSettings = CommonCommandsDiscriminant::CommonResResetSettings.discriminant(), // NO payload
    CommonReqStatus = CommonCommandsDiscriminant::CommonReqStatus.discriminant(), // NO payload
    CommonResStatus = CommonCommandsDiscriminant::CommonResStatus.discriminant(), // TODO: some status msg
    CommonReqSetVariable {
        payload: SetMsgPayload,
    } = CommonCommandsDiscriminant::CommonReqSetVariable.discriminant(), // SetMsg_t
    CommonResSetVariable {
        payload: SetMsgPayload,
    } = CommonCommandsDiscriminant::CommonResSetVariable.discriminant(), // SetMsg_t
    CommonReqGetVariable {
        payload: GetMsgPayload,
    } = CommonCommandsDiscriminant::CommonReqGetVariable.discriminant(), // GetMsg_t
    CommonResGetVariable {
        payload: SetMsgPayload,
    } = CommonCommandsDiscriminant::CommonResGetVariable.discriminant(), // SetMsg_t
    CommonTotalCmds,
}

#[derive(Debug, EnumDiscriminate)]
#[repr(u8)]
pub enum CommonCommandsDiscriminant {
    CommonReqResetSettings = 0,
    CommonResResetSettings = 1,
    CommonReqStatus = 2,
    CommonResStatus = 3,
    CommonReqSetVariable = 4,
    CommonResSetVariable = 5,
    CommonReqGetVariable = 6,
    CommonResGetVariable = 7,
    CommonTotalCmds = 8,
}

#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout, PartialEq, Clone)]
#[repr(C, packed)]
pub struct SetMsgPayload {
    pub variable_id: u8,
    pub value: u32,
}
#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout, PartialEq, Clone)]
#[repr(C, packed)]
pub struct GetMsgPayload {
    pub variable_id: u8,
}

#[test]
fn test_enum_discriminant() {
    use crate::protocol::commands::CommonCommandsDiscriminant;
    assert_eq!(
        CommonCommandsDiscriminant::CommonReqResetSettings.discriminant(),
        0
    );
    assert_eq!(
        CommonCommandsDiscriminant::CommonResResetSettings.discriminant(),
        1
    );
    assert_eq!(
        CommonCommandsDiscriminant::CommonReqStatus.discriminant(),
        2
    );
    assert_eq!(
        CommonCommandsDiscriminant::CommonResStatus.discriminant(),
        3
    );
    assert_eq!(
        CommonCommandsDiscriminant::CommonReqSetVariable.discriminant(),
        4
    );
    assert_eq!(
        CommonCommandsDiscriminant::CommonResSetVariable.discriminant(),
        5
    );
    assert_eq!(
        CommonCommandsDiscriminant::CommonReqGetVariable.discriminant(),
        6
    );
    assert_eq!(
        CommonCommandsDiscriminant::CommonResGetVariable.discriminant(),
        7
    );
    assert_eq!(
        CommonCommandsDiscriminant::CommonTotalCmds.discriminant(),
        8
    );
}
