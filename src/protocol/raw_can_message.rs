use modular_bitfield::prelude::{B5, B6};
use modular_bitfield::private::static_assertions;
use modular_bitfield::{bitfield, Specifier};
use std::mem::size_of;
use zerocopy_derive::{FromBytes, IntoBytes};

#[derive(Specifier)]
pub enum CanMessageDirection {
    MasterToNode,
    NodeToMaster,
}

#[derive(Specifier)]
pub enum CanMessagePriority {
    UrgentPriority,
    HighPriority,
    StandardPriority,
    LowPriority,
}
#[bitfield]
#[repr(u16)]
pub struct CanMessageId {
    pub direction: CanMessageDirection,
    pub node_id: B6,
    pub special_cmd: MessageSpecialCommand,
    pub priority: CanMessagePriority,
    #[skip]
    __: B5,
}
#[derive(Specifier)]
#[repr(i8)]
pub enum CanMessageBufferType {
    DirectBuffer,
    AbsoluteBuffer,
    RelativeBuffer,
    ReservedBuffer,
}

#[derive(Specifier)]
pub enum MessageSpecialCommand {
    AbortSpecialCmd,
    ClockSyncSpecialCmd, // DIR = MASTER2NODE_DIRECTION
    //TODO: This is from the legacy definition, but it doens't work in rust. Remove in future.
    //ErrorSpecialCmd = ClockSyncSpecialCmd as isize, DIR = NODE2MASTER_DIRECTION
    InfoSpecialCmd,
    StandardSpecialCmd,
}

#[bitfield]
#[derive(IntoBytes, FromBytes)]
#[repr(u8)]
pub struct CanMessageDataInfo {
    pub channel_id: B6,
    pub can_message_buffer: CanMessageBufferType,
}

#[derive(IntoBytes, FromBytes)]
#[repr(C, packed)]
pub struct CanMessageData {
    pub data_info: CanMessageDataInfo,
    pub command_id: u8,
    pub data: [u8; 62],
}

static_assertions::const_assert_eq!(size_of::<CanMessageData>(), 64);
