use crate::protocol::commands::{
    CommonCommands, CommonCommandsDiscriminant, GetMsgPayload, SetMsgPayload,
};
use crate::protocol::message::CommandTrait;
use crate::protocol::CanMessageData;
use anyhow::{anyhow, Error};
use ecu_emulator_macros_derive::EnumDiscriminate;
use zerocopy::{FromBytes, FromZeros, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, EnumDiscriminate, PartialEq)]
#[repr(isize)]
pub enum GenericCommand {
    GenericReqResetAllSettings =
        GenericCommandDiscriminant::GenericReqResetAllSettings.discriminant(), // NO payload
    GenericResResetAllSettings =
        GenericCommandDiscriminant::GenericResResetAllSettings.discriminant(), // NO payload
    GenericReqStatus = GenericCommandDiscriminant::GenericReqStatus.discriminant(), // NO payload
    GenericResStatus = GenericCommandDiscriminant::GenericResStatus.discriminant(), // TODO: some status msg
    GenericReqSetVariable {
        payload: SetMsgPayload,
    } = GenericCommandDiscriminant::GenericReqSetVariable.discriminant(), // SetMsg_t
    GenericResSetVariable {
        payload: SetMsgPayload,
    } = GenericCommandDiscriminant::GenericResSetVariable.discriminant(), // SetMsg_t
    GenericReqGetVariable {
        payload: GetMsgPayload,
    } = GenericCommandDiscriminant::GenericReqGetVariable.discriminant(), // GetMsg_t
    GenericResGetVariable {
        payload: SetMsgPayload,
    } = GenericCommandDiscriminant::GenericResGetVariable.discriminant(), // SetMsg_t
    GenericReqSyncClock = GenericCommandDiscriminant::GenericReqSyncClock.discriminant(), // NO FUCKING IDEA
    GenericResSyncClock = GenericCommandDiscriminant::GenericResSyncClock.discriminant(), // NO FUCKING IDEA
    GenericReqData = GenericCommandDiscriminant::GenericReqData.discriminant(), // NO payload
    GenericResData {
        payload: HeartBeatDataMsg,
    } = GenericCommandDiscriminant::GenericResData.discriminant(), // DataMsg_t
    GenericReqNodeInfo = GenericCommandDiscriminant::GenericReqNodeInfo.discriminant(), // NO payload
    GenericResNodeInfo {
        payload: NodeInfoMsg,
    } = GenericCommandDiscriminant::GenericResNodeInfo.discriminant(), // NodeInfoMsg_t
    GenericReqNodeStatus = GenericCommandDiscriminant::GenericReqNodeStatus.discriminant(), // NO payload
    GenericResNodeStatus = GenericCommandDiscriminant::GenericResNodeStatus.discriminant(), // NodeStatusMsg_t //TODO Ignoring parameters since it's unused. Remove in the future
    GenericReqSpeaker = GenericCommandDiscriminant::GenericReqSpeaker.discriminant(), // SpeakerMsg_t //TODO Ignoring parameters since it's unused. Remove in the future
    GenericReqThreshold = GenericCommandDiscriminant::GenericReqThreshold.discriminant(), // ThresholdMsg_t //TODO Ignoring parameters since it's unused. Remove in the future
    GenericReqFlashClear = GenericCommandDiscriminant::GenericReqFlashClear.discriminant(), // NO payload
    GenericResFlashStatus {
        status: u8,
    } = GenericCommandDiscriminant::GenericResFlashStatus.discriminant(), // FlashStatusMsg_t
    GenericTotalCmds = GenericCommandDiscriminant::GenericTotalCmds.discriminant(),
}
#[derive(EnumDiscriminate)]
#[repr(isize)]
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

fn copy_bytes_to_array<const N: usize>(src: &[u8]) -> [u8; N] {
    let mut dest = [0u8; N];
    let len = src.len().min(N);
    dest[..len].copy_from_slice(&src[..len]);
    dest
}

impl TryFrom<CanMessageData> for GenericCommand {
    type Error = Error;
    fn try_from(value: CanMessageData) -> Result<Self, Self::Error> {
        let command_id = value.command_id;
        match command_id as isize {
            x if x == GenericCommandDiscriminant::GenericReqResetAllSettings as isize => {
                Ok(GenericCommand::GenericReqResetAllSettings)
            }
            x if x == GenericCommandDiscriminant::GenericResResetAllSettings as isize => {
                Ok(GenericCommand::GenericResResetAllSettings)
            }
            x if x == GenericCommandDiscriminant::GenericReqStatus as isize => {
                Ok(GenericCommand::GenericReqStatus)
            }
            x if x == GenericCommandDiscriminant::GenericResStatus as isize => {
                Ok(GenericCommand::GenericResStatus)
            }
            x if x == GenericCommandDiscriminant::GenericReqSetVariable as isize => {
                Ok(GenericCommand::GenericReqSetVariable {
                    payload: SetMsgPayload::read_from_prefix(&value.data[..])
                        .map_err(|e| anyhow!("Failed to parse SetMsgPayload: {}", e))?
                        .0,
                })
            }
            x if x == GenericCommandDiscriminant::GenericResSetVariable as isize => {
                Ok(GenericCommand::GenericResSetVariable {
                    payload: SetMsgPayload::read_from_prefix(&value.data[..])
                        .map_err(|e| anyhow!("Failed to parse SetMsgPayload: {}", e))?
                        .0,
                })
            }
            x if x == GenericCommandDiscriminant::GenericReqGetVariable as isize => {
                Ok(GenericCommand::GenericReqGetVariable {
                    payload: GetMsgPayload::read_from_prefix(&value.data[..])
                        .map_err(|e| anyhow!("Failed to parse GetMsgPayload: {}", e))?
                        .0,
                })
            }
            x if x == GenericCommandDiscriminant::GenericResGetVariable as isize => {
                Ok(GenericCommand::GenericResGetVariable {
                    payload: SetMsgPayload::read_from_prefix(&value.data[..])
                        .map_err(|e| anyhow!("Failed to parse SetMsgPayload: {}", e))?
                        .0,
                })
            }
            x if x == GenericCommandDiscriminant::GenericReqSyncClock as isize => {
                Ok(GenericCommand::GenericReqSyncClock)
            }
            x if x == GenericCommandDiscriminant::GenericResSyncClock as isize => {
                Ok(GenericCommand::GenericResSyncClock)
            }
            x if x == GenericCommandDiscriminant::GenericReqData as isize => {
                Ok(GenericCommand::GenericReqData)
            }
            x if x == GenericCommandDiscriminant::GenericResData as isize => {
                Ok(GenericCommand::GenericResData {
                    payload: HeartBeatDataMsg::read_from_prefix(&value.data[..])
                        .map_err(|e| anyhow!("Failed to parse HeartBeatDataMsg: {}", e))?
                        .0,
                })
            }
            x if x == GenericCommandDiscriminant::GenericReqNodeInfo as isize => {
                Ok(GenericCommand::GenericReqNodeInfo)
            }
            x if x == GenericCommandDiscriminant::GenericResNodeInfo as isize => {
                Ok(GenericCommand::GenericResNodeInfo {
                    payload: NodeInfoMsg::read_from_prefix(&value.data[..])
                        .map_err(|e| anyhow!("Failed to parse NodeInfoMsg: {}", e))?
                        .0,
                })
            }
            x if x == GenericCommandDiscriminant::GenericReqNodeStatus as isize => {
                Ok(GenericCommand::GenericReqNodeStatus)
            }
            x if x == GenericCommandDiscriminant::GenericResNodeStatus as isize => {
                Ok(GenericCommand::GenericResNodeStatus)
            }
            x if x == GenericCommandDiscriminant::GenericReqSpeaker as isize => {
                Ok(GenericCommand::GenericReqSpeaker)
            }
            x if x == GenericCommandDiscriminant::GenericReqThreshold as isize => {
                Ok(GenericCommand::GenericReqThreshold)
            }
            x if x == GenericCommandDiscriminant::GenericReqFlashClear as isize => {
                Ok(GenericCommand::GenericReqFlashClear)
            }
            x if x == GenericCommandDiscriminant::GenericResFlashStatus as isize => {
                Ok(GenericCommand::GenericResFlashStatus {
                    status: value.data[0],
                })
            }
            _ => Err(anyhow!("Invalid GenericCommand id: {command_id}")),
        }
    }
}
impl CommandTrait for GenericCommand {
    fn as_can_message_data(&self) -> CanMessageData {
        let mut data = match self {
            GenericCommand::GenericReqResetAllSettings => CanMessageData::new_zeroed(),
            GenericCommand::GenericResResetAllSettings => CanMessageData::new_zeroed(),
            GenericCommand::GenericReqStatus => CanMessageData::new_zeroed(),
            GenericCommand::GenericResStatus => CanMessageData::new_zeroed(),
            GenericCommand::GenericReqSetVariable { payload }
            | GenericCommand::GenericResSetVariable { payload }
            | GenericCommand::GenericResGetVariable { payload } => {
                let mut data = CanMessageData::new_zeroed();
                data.data = copy_bytes_to_array(payload.as_bytes());
                data
            }
            GenericCommand::GenericReqGetVariable { payload } => {
                let mut data = CanMessageData::new_zeroed();
                data.data = copy_bytes_to_array(payload.as_bytes());
                data
            }
            GenericCommand::GenericReqSyncClock => CanMessageData::new_zeroed(),
            GenericCommand::GenericResSyncClock => CanMessageData::new_zeroed(),
            GenericCommand::GenericReqData => CanMessageData::new_zeroed(),
            GenericCommand::GenericResData { payload } => {
                let mut data = CanMessageData::new_zeroed();
                data.data = copy_bytes_to_array(payload.as_bytes());
                data
            }
            GenericCommand::GenericReqNodeInfo => CanMessageData::new_zeroed(),
            GenericCommand::GenericResNodeInfo { payload } => {
                let mut data = CanMessageData::new_zeroed();
                data.data = <[u8; 62]>::try_from((&payload).as_bytes()).unwrap(); //TODO is it smart to ignore the potential error here?
                data
            }
            GenericCommand::GenericReqNodeStatus => CanMessageData::new_zeroed(),
            GenericCommand::GenericResNodeStatus => CanMessageData::new_zeroed(),
            GenericCommand::GenericReqSpeaker => CanMessageData::new_zeroed(),
            GenericCommand::GenericReqThreshold => CanMessageData::new_zeroed(),
            GenericCommand::GenericReqFlashClear => CanMessageData::new_zeroed(),
            GenericCommand::GenericResFlashStatus { status } => {
                let mut data = CanMessageData::new_zeroed();
                data.data[0] = *status;
                data
            }
            GenericCommand::GenericTotalCmds => CanMessageData::new_zeroed(),
        };
        data.command_id = self.discriminant() as u8;
        data
    }
}

#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout, PartialEq)]
#[repr(C, packed)]
pub struct NodeInfoMsg {
    pub firmware_version: u32,
    pub channel_mask: u32,
    pub channel_type: [u8; 32],
}

#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout, PartialEq)]
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
