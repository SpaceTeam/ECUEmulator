use crate::protocol::commands::{
    CommonCommands, CommonCommandsDiscriminant, GetMsgPayload, SetMsgPayload,
};
use crate::protocol::message::CommandTrait;
use crate::protocol::CanMessageData;
use ecu_emulator_macros_derive::EnumDiscriminate;
use zerocopy::{FromZeros, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes};

#[derive(Debug, EnumDiscriminate)]
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

#[derive(Debug, FromBytes, IntoBytes, Immutable)]
#[repr(C, packed)]
pub struct NodeInfoMsg {
    firmware_version: u32,
    channel_mask: u32,
    channel_type: [u8; 32],
}

#[derive(Debug, FromBytes, IntoBytes, Immutable)]
#[repr(C, packed)]
pub struct HeartBeatDataMsg {
    pub channel_mask: u32,
    pub data: [u8; 60],
}
//Is 64 since for the DataMsg the whole can data section is used instead of also transmitting the data info and command id
static_assertions::const_assert_eq!(size_of::<HeartBeatDataMsg>(), 64);
