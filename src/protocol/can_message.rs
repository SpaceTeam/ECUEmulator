use crate::protocol::payloads;
use ecu_emulator_macros::padded_enum;
use ecu_emulator_macros_derive::EnumDiscriminate;

padded_enum! {

(size = 64)

#[derive(Debug, EnumDiscriminate, PartialEq, Clone)]
#[repr(u8)]
pub enum CanMessage {
    // Node Discovery and Information
    #[pad(63)]
    NodeInfoReq = 0, // NO payload
    #[pad(0)]
    NodeInfoAnnouncement {
        payload: payloads::NodeInfoResPayload,
    } = 1,

    // Status Messages
    #[pad(0)]
    InfoStatus {
        payload: payloads::StatusPayload
    } = 10,
    #[pad(0)]
    WarningStatus {
        payload: payloads::StatusPayload
    } = 11,
    #[pad(0)]
    ErrorStatus {
       payload: payloads::StatusPayload
    } = 12,

    // Field Registration
    #[pad(0)]
    TelemetryValueRegistration {
        payload: payloads::FieldRegistrationPayload,
    } = 20,
    #[pad(0)]
    ParameterRegistration {
        payload: payloads::FieldRegistrationPayload,
    } = 21,

    // Telemetry Group Management
    #[pad(0)]
    TelemetryGroupDefinition {
        payload: payloads::TelemetryGroupDefinitionPayload,
    } = 30,
    #[pad(0)]
    TelemetryGroupUpdate {
        payload: payloads::TelemetryGroupUpdatePayload,
    } = 31,

    // Heartbeat
    #[pad(59)]
    HeartbeatReq {
        payload: payloads::HeartbeatPayload
    } = 40,
    #[pad(59)]
    HeartbeatRes {
        payload: payloads::HeartbeatPayload
    } = 41,

    // Parameter Management
    #[pad(1)]
    ParameterSetReq {
        payload: payloads::ParameterSetReqPayload,
    } = 50,
    #[pad(0)]
    ParameterSetConfirmation {
        payload: payloads::ParameterSetConfirmationPayload,
    } = 51,
    #[pad(61)]
    ParameterSetLockReq {
        payload: payloads::ParameterSetLockPayload,
    } = 52,
    #[pad(61)]
    ParameterSetLockConfirmation {
        payload: payloads::ParameterSetLockPayload,
    } = 53,

    // Field Access
    #[pad(62)]
    FieldGetReq {
        payload: payloads::FieldGetReqPayload,
    } = 60,
    #[pad(0)]
    FieldGetRes {
        payload: payloads::FieldGetResPayload,
    } = 61,
    #[pad(2)]
    FieldIDLookupReq {
        payload: payloads::FieldIDLookupReqPayload,
    } = 62,
    #[pad(61)]
    FieldIDLookupRes {
        payload: payloads::FieldIDLookupResPayload,
    } = 63,
}

}
