use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ProtocolError {
    #[error("Unknown protocol")]
    UnknownProtocol,

    #[error("MQTT protocol error: {0}")]
    MQTTProtocolError(#[from] MQTTProtocolError),
    
}

#[cfg(test)]
mod protocol_error_tests {
    use super::*;

    #[test]
    fn protocol_error_unknown_protocol() {
        let error = ProtocolError::UnknownProtocol;
        assert_eq!(format!("{}", error), "Unknown protocol");
    }

    #[test]
    fn protocol_error_mqtt_protocol_error() {
        let mqtt_error = MQTTProtocolError::InvalidPacketType;
        let protocol_error: ProtocolError = mqtt_error.into();
        assert_eq!(
            format!("{}", protocol_error),
            "MQTT protocol error: Invalid packet type: reserved bits are forbidden to use"
        );
    }

}
