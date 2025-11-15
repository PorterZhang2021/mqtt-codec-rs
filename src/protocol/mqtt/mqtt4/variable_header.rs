use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::protocol_error::ProtocolError;
use crate::protocol::utils::utf::utf_8_handler;
use bytes::BytesMut;

pub enum VariableHeader {
    Connect {
        protocol_level: u8,
        user_name_flag: bool,
        password_flag: bool,
        will_retain: bool,
        will_qos: u8,
        will_flag: bool,
        clean_session: bool,
        keep_alive: u16,
    },
    ConnAck,
    Publish,
    PubAck,
    PubRec,
    PubRel,
    PubComp,
    Subscribe,
    SubAck,
    Unsubscribe,
    UnsubAck,
    PingReq,
    PingResp,
    Disconnect,
}

impl VariableHeader {
    pub(super) fn verify_protocol_name(
        bytes: &mut impl ByteOperations,
    ) -> Result<(), MQTTProtocolError> {
        let protocol_name = utf_8_handler::parse(bytes)?;
        if protocol_name != "MQTT" {
            return Err(MQTTProtocolError::ProtocolNameError(protocol_name));
        }
        Ok(())
    }
}

impl VariableHeader {}
#[cfg(test)]
mod variable_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::mqtt::mqtt4::variable_header::VariableHeader;
    use crate::protocol::utils::utf::utf_8_handler::write_utf8_for_mqtt;
    use bytes::BytesMut;

    // todo connect
    // todo connect contain protocol name
    #[test]
    fn connect_contain_protocol_name() {
        let mut bytes_mut = BytesMut::new();
        write_utf8_for_mqtt(&mut bytes_mut, "MQTT");
        VariableHeader::verify_protocol_name(&mut bytes_mut).unwrap();

        // assert_eq!(protocol_name, "MQTT");
    }
    // todo connect contain protocol level
    // todo connect contain connect flags
    // todo connect keep alive
    // todo connack
    // todo publish
    // todo puback
    // todo pubrec
    // todo pubrel
    // todo pubcomp
    // todo subscribe
    // todo suback
    // todo unsubscribe
    // todo unsuback
    // todo pingreq
    // todo pingresp
    // todo disconnect
}
