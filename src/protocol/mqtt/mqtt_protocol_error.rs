#[derive(Debug, thiserror::Error)]
pub enum MQTTProtocolError {
    #[error("Invalid packet type: reserved bits are forbidden to use")]
    InvalidPacketType,
    #[error("This Control Packet type reserved flag is invalid")]
    InvalidFixedHeaderFlags,
    #[error("This Control Packet type reserved flag can not be used")]
    FixedHeaderFlagsNotUsed,
    #[error("QoS can support 0, 1, 2, the specified QoS {0} level is not supported")]
    QoSLevelNotSupported(u8),
}
