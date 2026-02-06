use modular_bitfield::prelude::B5;
use modular_bitfield::private::static_assertions;
use modular_bitfield::{bitfield, Specifier};
use std::mem::size_of;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Specifier, Debug, PartialEq, Eq)]
pub enum CanMessagePriority {
    Low = 0,
    High = 1,
}

#[bitfield]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
pub struct CanMessageId {
    pub receiver_id: B5,
    pub sender_id: B5,
    pub priority: CanMessagePriority,
    #[skip]
    __: B5,
}

#[derive(Debug, IntoBytes, FromBytes, Immutable, KnownLayout)]
#[repr(C, packed)]
pub struct CanMessageFrame {
    pub message_type: u8,
    pub data: [u8; 63],
}

static_assertions::const_assert_eq!(size_of::<CanMessageFrame>(), 64);
