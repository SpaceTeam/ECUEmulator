use crate::protocol::commands::{
    CommonCommands, CommonCommandsDiscriminant, GetMsgPayload, SetMsgPayload,
};
use crate::protocol::message::CommandTrait;
use crate::protocol::CanMessageData;
use anyhow::{anyhow, Error};
use ecu_emulator_macros::padded_enum;
use ecu_emulator_macros_derive::EnumDiscriminate;
use zerocopy::{FromBytes, FromZeros, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

padded_enum! {

(size = 63)

#[derive(Debug, EnumDiscriminate, PartialEq, Clone)]
#[repr(u8)]
pub enum GenericCommand {
    #[pad(62)]
    GenericReqResetAllSettings =
        GenericCommandDiscriminant::GenericReqResetAllSettings.discriminant(), // NO payload
    #[pad(62)]
    GenericResResetAllSettings =
        GenericCommandDiscriminant::GenericResResetAllSettings.discriminant(), // NO payload
    #[pad(62)]
    GenericReqStatus = GenericCommandDiscriminant::GenericReqStatus.discriminant(), // NO payload
    #[pad(62)]
    GenericResStatus = GenericCommandDiscriminant::GenericResStatus.discriminant(), // TODO: some status msg
    #[pad(57)]
    GenericReqSetVariable {
        payload: SetMsgPayload,
    } = GenericCommandDiscriminant::GenericReqSetVariable.discriminant(), // SetMsg_t
    #[pad(57)]
    GenericResSetVariable {
        payload: SetMsgPayload,
    } = GenericCommandDiscriminant::GenericResSetVariable.discriminant(), // SetMsg_t
    #[pad(61)]
    GenericReqGetVariable {
        payload: GetMsgPayload,
    } = GenericCommandDiscriminant::GenericReqGetVariable.discriminant(), // GetMsg_t
    #[pad(57)]
    GenericResGetVariable {
        payload: SetMsgPayload,
    } = GenericCommandDiscriminant::GenericResGetVariable.discriminant(), // SetMsg_t
    #[pad(62)]
    GenericReqSyncClock = GenericCommandDiscriminant::GenericReqSyncClock.discriminant(), // NO FUCKING IDEA
    #[pad(62)]
    GenericResSyncClock = GenericCommandDiscriminant::GenericResSyncClock.discriminant(), // NO FUCKING IDEA
    #[pad(62)]
    GenericReqData = GenericCommandDiscriminant::GenericReqData.discriminant(), // NO payload
    #[pad(2)]
    GenericResData {
        payload: HeartBeatDataMsg,
    } = GenericCommandDiscriminant::GenericResData.discriminant(), // DataMsg_t
    #[pad(62)]
    GenericReqNodeInfo = GenericCommandDiscriminant::GenericReqNodeInfo.discriminant(), // NO payload
    #[pad(22)]
    GenericResNodeInfo {
        payload: NodeInfoMsg,
    } = GenericCommandDiscriminant::GenericResNodeInfo.discriminant(), // NodeInfoMsg_t
    #[pad(62)]
    GenericReqNodeStatus = GenericCommandDiscriminant::GenericReqNodeStatus.discriminant(), // NO payload
    #[pad(62)]
    GenericResNodeStatus = GenericCommandDiscriminant::GenericResNodeStatus.discriminant(), // NodeStatusMsg_t //TODO Ignoring parameters since it's unused. Remove in the future
    #[pad(62)]
    GenericReqSpeaker = GenericCommandDiscriminant::GenericReqSpeaker.discriminant(), // SpeakerMsg_t //TODO Ignoring parameters since it's unused. Remove in the future
    #[pad(62)]
    GenericReqThreshold = GenericCommandDiscriminant::GenericReqThreshold.discriminant(), // ThresholdMsg_t //TODO Ignoring parameters since it's unused. Remove in the future
    #[pad(62)]
    GenericReqFlashClear = GenericCommandDiscriminant::GenericReqFlashClear.discriminant(), // NO payload
    #[pad(61)]
    GenericResFlashStatus {
        status: u8,
    } = GenericCommandDiscriminant::GenericResFlashStatus.discriminant(), // FlashStatusMsg_t
    #[pad(62)]
    GenericTotalCmds = GenericCommandDiscriminant::GenericTotalCmds.discriminant(),
}
}
#[derive(EnumDiscriminate)]
#[repr(u8)]
pub enum GenericCommandDiscriminant {
    GenericReqResetAllSettings = CommonCommands::CommonReqResetSettings.discriminant(),
    GenericResResetAllSettings = CommonCommands::CommonResResetSettings.discriminant(),
    GenericReqStatus = CommonCommands::CommonReqStatus.discriminant(),
    GenericResStatus = CommonCommands::CommonResStatus.discriminant(),
    GenericReqSetVariable = CommonCommandsDiscriminant::CommonReqSetVariable.discriminant(),
    GenericResSetVariable = CommonCommandsDiscriminant::CommonResSetVariable.discriminant(),
    GenericReqGetVariable = CommonCommandsDiscriminant::CommonReqGetVariable.discriminant(),
    GenericResGetVariable = CommonCommandsDiscriminant::CommonResGetVariable.discriminant(),
    GenericReqSyncClock = CommonCommandsDiscriminant::CommonTotalCmds.discriminant(),
    GenericResSyncClock = 9,
    GenericReqData = 10,
    GenericResData = 11,
    GenericReqNodeInfo = 12,
    GenericResNodeInfo = 13,
    GenericReqNodeStatus = 14,
    GenericResNodeStatus = 15,
    GenericReqSpeaker = 16,
    GenericReqThreshold = 17,
    GenericReqFlashClear = 18,
    GenericResFlashStatus = 19,
    GenericTotalCmds = 20,
}

#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout, PartialEq, Clone)]
#[repr(C, packed)]
pub struct NodeInfoMsg {
    pub firmware_version: u32,
    pub channel_mask: u32,
    pub channel_type: [u8; 32],
}

#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout, PartialEq, Clone)]
#[repr(C, packed)]
pub struct HeartBeatDataMsg {
    pub channel_mask: u32,
    pub data: [u8; 56],
}
#[derive(Debug)]
pub enum FlashStatus {
    Initiated, //returns when flash clear started
    Completed, //returns when flash clear finished
    Full,      //returns when flash is full
}

//Is 60 since for the DataMsg the whole can data section is used instead of also transmitting the data info and command id
static_assertions::const_assert_eq!(size_of::<HeartBeatDataMsg>(), 60);
