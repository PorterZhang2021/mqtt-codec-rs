use crate::protocol::utils::code_error::CodeError;

#[derive(Debug, thiserror::Error)]
pub enum MQTTProtocolError {
    #[error("Malformed packet")]
    MalformedPacket,

    #[error("Invalid packet type: reserved bits are forbidden to use")]
    InvalidPacketType,

    #[error("This Control Packet type reserved flag is invalid")]
    InvalidFixedHeaderFlags,

    #[error("QoS can support 0, 1, 2, the specified QoS {0} level is not supported")]
    QoSLevelNotSupported(u8),

    #[error("Remaining Length field is malformed")]
    MalformedRemainingLength,

    #[error("Packet does not have enough bytes")]
    PacketTooShort,

    #[error("Protocol Name error: {0}")]
    ProtocolNameError(String),

    #[error("from CodeError: {0}")]
    CodeError(#[from] CodeError),
}
