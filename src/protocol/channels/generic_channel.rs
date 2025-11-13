use crate::protocol::commands::{GetMsgPayload, SetMsgPayload};
use crate::protocol::message::CommandTrait;
use crate::protocol::CanMessageData;

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
    GenericResNodeStatus, // NodeStatusMsg_t //TODO Ignoring since it's unused. Remove in the future
    GenericReqSpeaker,    // SpeakerMsg_t //TODO Ignoring since it's unused. Remove in the future
    GenericReqThreshold,  // ThresholdMsg_t //TODO Ignoring since it's unused. Remove in the future
    GenericReqFlashClear, // NO payload
    GenericResFlashStatus { status: u8 }, // FlashStatusMsg_t //TODO find a way to
    GenericTotalCmds,
}

impl CommandTrait for GenericCommand {
    fn as_can_message_data(&self) -> CanMessageData {
        todo!()
    }
}

#[derive(Debug)]
#[repr(packed, C)]
pub struct NodeInfoMsg {
    firmware_version: u32,
    channel_mask: u32,
    channel_type: [u8; 32],
}

#[derive(Debug)]
#[repr(packed, C)]
pub struct HeartBeatDataMsg {
    pub channel_mask: u32,
    pub data: [u8; 60],
}
//Is 64 since for the DataMsg the whole can data section is used instead of also transmitting the data info and command id
static_assertions::const_assert_eq!(size_of::<HeartBeatDataMsg>(), 64);
