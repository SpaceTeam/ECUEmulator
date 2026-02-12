use modular_bitfield::{private::static_assertions, Specifier};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, TryFromBytes};

#[derive(Specifier, Debug, Copy, Clone, PartialEq, Eq, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum CanDataType {
    Float32 = 0,
    Int32 = 1,
    Int16 = 2,
    Int8 = 3,
    UInt32 = 4,
    UInt16 = 5,
    UInt8 = 6,
    Boolean = 7,
}

#[derive(Specifier, Debug, Copy, Clone, PartialEq, Eq, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum ParameterSetStatus {
    Success = 0,                // Parameter was successfully set
    InvalidParameterID = 1,     // The parameter ID does not exist
    ParameterLocked = 2,        // The parameter is locked and cannot be modified
    NodeToNodeModification = 3, // The parameter was modified by another node
}

#[derive(Specifier, Debug, Copy, Clone, PartialEq, Eq, Immutable, TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum ParameterLockStatus {
    Unlocked = 0,
    Locked = 1,
}

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct NodeInfoResPayload {
    pub tel_count: u8,         // Number of telemetry_values on this node
    pub par_count: u8,         // Number of parameters on this node
    pub firmware_hash: u32,    // Hash of the firmware version
    pub liquid_hash: u32,      // Hash of the LiquidCan protocol version
    pub device_name: [u8; 53], // Human-readable device name
}

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct StatusPayload {
    pub msg: [u8; 63], // Status message text
}

// Important: only derives TryFromBytes because enum CanDataType doesn't cover all possible enum variants for u8
#[derive(Debug, Clone, TryFromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct FieldRegistrationPayload {
    pub field_id: u8,            // Unique identifier for this field
    pub field_type: CanDataType, // Data type
    pub field_name: [u8; 61],    // Human-readable field name
}

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct TelemetryGroupDefinitionPayload {
    pub group_id: u8,        // Unique identifier for this group
    pub field_ids: [u8; 62], // Array of field IDs in this group
}

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct TelemetryGroupUpdatePayload {
    pub group_id: u8,     // Group identifier
    pub values: [u8; 62], // Packed values of all telemetry values in the group
}

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct HeartbeatPayload {
    pub counter: u32, // Incrementing counter value
}

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct ParameterSetReqPayload {
    pub parameter_id: u8, // Parameter identifier
    pub value: [u8; 61],  // New value (type depends on parameter)
}

// Important: only derives TryFromBytes because enum ParameterSetStatus doesn't cover all possible enum variants for u8
#[derive(Debug, Clone, TryFromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct ParameterSetConfirmationPayload {
    pub parameter_id: u8,           // Parameter identifier
    pub status: ParameterSetStatus, // Status code
    pub value: [u8; 61],            // Confirmed value after set operation
}

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct FieldGetReqPayload {
    pub field_id: u8, // Field identifier
}

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct FieldGetResPayload {
    pub field_id: u8,    // Field identifier
    pub value: [u8; 62], // Field value
}

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct FieldIDLookupReqPayload {
    pub field_name: [u8; 61], // Field name
}

// Important: only derives TryFromBytes because enum CanDataType doesn't cover all possible enum variants for u8
#[derive(Debug, Clone, TryFromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct FieldIDLookupResPayload {
    pub field_id: u8,            // Field ID
    pub field_type: CanDataType, // Field Datatype
}

// Important: only derives TryFromBytes because bool doesn't derive FromBytes
#[derive(Debug, Clone, TryFromBytes, IntoBytes, Immutable, PartialEq)]
#[repr(C, packed)]
pub struct ParameterSetLockPayload {
    pub parameter_id: u8,                    // Parameter identifier to lock
    pub parameter_lock: ParameterLockStatus, // Lock status (0=unlocked, 1=locked)
}

static_assertions::const_assert_eq!(size_of::<NodeInfoResPayload>(), 63);
static_assertions::const_assert_eq!(size_of::<StatusPayload>(), 63);
static_assertions::const_assert_eq!(size_of::<FieldRegistrationPayload>(), 63);
static_assertions::const_assert_eq!(size_of::<TelemetryGroupDefinitionPayload>(), 63);
static_assertions::const_assert_eq!(size_of::<TelemetryGroupUpdatePayload>(), 63);
static_assertions::const_assert_eq!(size_of::<HeartbeatPayload>(), 4);
static_assertions::const_assert_eq!(size_of::<ParameterSetReqPayload>(), 62);
static_assertions::const_assert_eq!(size_of::<ParameterSetConfirmationPayload>(), 63);
static_assertions::const_assert_eq!(size_of::<FieldGetReqPayload>(), 1);
static_assertions::const_assert_eq!(size_of::<FieldGetResPayload>(), 63);
static_assertions::const_assert_eq!(size_of::<FieldIDLookupReqPayload>(), 61);
static_assertions::const_assert_eq!(size_of::<FieldIDLookupResPayload>(), 2);
static_assertions::const_assert_eq!(size_of::<ParameterSetLockPayload>(), 2);
