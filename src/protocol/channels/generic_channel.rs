use crate::protocol::commands::{GetMsgPayload, SetMsgPayload};
use crate::protocol::message::CommandTrait;
use crate::protocol::CanMessageData;
use zerocopy::{FromZeros, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes};

/*#[derive(Debug)]
#[repr(isize)]
//TODO @Michael do you know of a way to make the enum variants have payloads and also have discriminants?
pub enum GenericCommand{
    GenericReqResetAllSettings = CommonReqResetSettings as isize,	// NO payload
    GenericResResetAllSettings = CommonResResetSettings as isize,	// NO payload
    GenericReqStatus = CommonReqStatus as isize,					// NO payload
    GenericResStatus = CommonResStatus as isize,					// TODO: some status msg
    GenericReqSetVariable{payload: SetMsgPayload} = CommonReqSetVariable as isize,		// SetMsg_t
    GenericResSetVariable{payload: SetMsgPayload} = CommonResSetVariable as isize,		    // SetMsg_t
    GenericReqGetVariable{payload: GetMsgPayload} = CommonReqGetVariable as isize,		    // GetMsg_t
    GenericResGetVariable{payload: SetMsgPayload} = CommonResGetVariable as isize,		    // SetMsg_t
    GenericReqSyncClock = CommonTotalCmds as isize,				    // NO FUCKING IDEA
    GenericResSyncClock,								        	// NO FUCKING IDEA
    GenericReqData,									            	// NO payload
    GenericResData{payload: HeartBeatDataMsg},                      // DataMsg_t
    GenericReqNodeInfo,								            	// NO payload
    GenericResNodeInfo{payload: NodeInfoMsg},					    // NodeInfoMsg_t
    GenericReqNodeStatus,							            	// NO payload
    GenericResNodeStatus,							            	// NodeStatusMsg_t //TODO Ignoring since it's unused. Remove in the future
    GenericReqSpeaker,								            	// SpeakerMsg_t //TODO Ignoring since it's unused. Remove in the future
    GenericReqThreshold,							                // ThresholdMsg_t //TODO Ignoring since it's unused. Remove in the future
    GenericReqFlashClear,							            	// NO payload
    GenericResFlashStatus{status: u8},                              // FlashStatusMsg_t //TODO find a way to
    GenericTotalCmds
}*/

#[derive(Debug)]
#[repr(isize)]
//TODO This is "Wrong" since the enum variants with payloads do not have discriminants. Find a way to fix this in the future.
pub enum GenericCommand {
    GenericReqResetAllSettings,                       // NO payload
    GenericResResetAllSettings,                       // NO payload
    GenericReqStatus,                                 // NO payload
    GenericResStatus,                                 // TODO: some status msg
    GenericReqSetVariable { payload: SetMsgPayload }, // SetMsg_t
    GenericResSetVariable { payload: SetMsgPayload }, // SetMsg_t
    GenericReqGetVariable { payload: GetMsgPayload }, // GetMsg_t
    GenericResGetVariable { payload: SetMsgPayload }, // SetMsg_t
    GenericReqSyncClock,                              // NO FUCKING IDEA
    GenericResSyncClock,                              // NO FUCKING IDEA
    GenericReqData,                                   // NO payload
    GenericResData { payload: HeartBeatDataMsg },     // DataMsg_t
    GenericReqNodeInfo,                               // NO payload
    GenericResNodeInfo { payload: NodeInfoMsg },      // NodeInfoMsg_t
    GenericReqNodeStatus,                             // NO payload
    GenericResNodeStatus, // NodeStatusMsg_t //TODO Ignoring parameters since it's unused. Remove in the future
    GenericReqSpeaker,    // SpeakerMsg_t //TODO Ignoring parameters since it's unused. Remove in the future
    GenericReqThreshold,  // ThresholdMsg_t //TODO Ignoring parameters since it's unused. Remove in the future
    GenericReqFlashClear, // NO payload
    GenericResFlashStatus { status: u8 }, // FlashStatusMsg_t //TODO find a way to
    GenericTotalCmds,
}
pub enum GenericCommandDiscriminant {
    GenericReqResetAllSettings = 0,
    GenericResResetAllSettings = 1,
    GenericReqStatus = 2,
    GenericResStatus = 3,
    GenericReqSetVariable = 4,
    GenericResSetVariable = 5,
    GenericReqGetVariable = 6,
    GenericResGetVariable = 7,
    GenericReqSyncClock = 8,
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
impl From<&GenericCommand> for u8 {
    fn from(value: &GenericCommand) -> Self {
        match value {
            GenericCommand::GenericReqResetAllSettings => GenericCommandDiscriminant::GenericReqResetAllSettings as u8,
            GenericCommand::GenericResResetAllSettings => GenericCommandDiscriminant::GenericResResetAllSettings as u8,
            GenericCommand::GenericReqStatus => GenericCommandDiscriminant::GenericReqStatus as u8,
            GenericCommand::GenericResStatus => GenericCommandDiscriminant::GenericResStatus as u8,
            GenericCommand::GenericReqSetVariable { .. } => GenericCommandDiscriminant::GenericReqSetVariable as u8,
            GenericCommand::GenericResSetVariable { .. } => GenericCommandDiscriminant::GenericResSetVariable as u8,
            GenericCommand::GenericReqGetVariable { .. } => GenericCommandDiscriminant::GenericReqGetVariable as u8,
            GenericCommand::GenericResGetVariable { .. } => GenericCommandDiscriminant::GenericResGetVariable as u8,
            GenericCommand::GenericReqSyncClock => GenericCommandDiscriminant::GenericReqSyncClock as u8,
            GenericCommand::GenericResSyncClock => GenericCommandDiscriminant::GenericResSyncClock as u8,
            GenericCommand::GenericReqData => GenericCommandDiscriminant::GenericReqData as u8,
            GenericCommand::GenericResData { .. } => GenericCommandDiscriminant::GenericResData as u8,
            GenericCommand::GenericReqNodeInfo => GenericCommandDiscriminant::GenericReqNodeInfo as u8,
            GenericCommand::GenericResNodeInfo { .. } => GenericCommandDiscriminant::GenericResNodeInfo as u8,
            GenericCommand::GenericReqNodeStatus => GenericCommandDiscriminant::GenericReqNodeStatus as u8,
            GenericCommand::GenericResNodeStatus => GenericCommandDiscriminant::GenericResNodeStatus as u8,
            GenericCommand::GenericReqSpeaker => GenericCommandDiscriminant::GenericReqSpeaker as u8,
            GenericCommand::GenericReqThreshold => GenericCommandDiscriminant::GenericReqThreshold as u8,
            GenericCommand::GenericReqFlashClear => GenericCommandDiscriminant::GenericReqFlashClear as u8,
            GenericCommand::GenericResFlashStatus { .. } => GenericCommandDiscriminant::GenericResFlashStatus as u8,
            GenericCommand::GenericTotalCmds => GenericCommandDiscriminant::GenericTotalCmds as u8,
        }
    }
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
            GenericCommand::GenericReqSetVariable { payload } |
            GenericCommand::GenericResSetVariable { payload } |
            GenericCommand::GenericResGetVariable { payload }=> {
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
                data.data = <[u8; 62]>::try_from((&payload).as_bytes()).unwrap();//TODO is it smart to ignore the potential error here?
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
            GenericCommand::GenericTotalCmds => CanMessageData::new_zeroed()
        };
        data.command_id = self.into();
        data
    }
}

#[derive(Debug)]
#[derive(FromBytes,IntoBytes,Immutable)]
#[repr(C, packed)]
pub struct NodeInfoMsg {
    firmware_version: u32,
    channel_mask: u32,
    channel_type: [u8; 32],
}

#[derive(Debug)]
#[derive(FromBytes,IntoBytes,Immutable)]
#[repr(C, packed)]
pub struct HeartBeatDataMsg {
    pub channel_mask: u32,
    pub data: [u8; 60],
}
//Is 64 since for the DataMsg the whole can data section is used instead of also transmitting the data info and command id
static_assertions::const_assert_eq!(size_of::<HeartBeatDataMsg>(), 64);
