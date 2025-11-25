use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::mqtt_utils;

pub(crate) struct UnSubAckVariableHeader {
    packet_identifier: u16,
}
impl UnSubAckVariableHeader {
    fn parse(bytes: &mut impl ByteOperations) -> Result<UnSubAckVariableHeader, MQTTProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(UnSubAckVariableHeader { packet_identifier })
    }
}

#[cfg(test)]
mod unsubscribe_variable_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::mqtt::mqtt4::variable_header_parser::unsub_ack::UnSubAckVariableHeader;
    use bytes::BytesMut;

    #[test]
    fn unsub_ack_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0x22);
        bytes.write_a_byte(0x11);

        let unsubscribe_ack_variable_header = UnSubAckVariableHeader::parse(&mut bytes).unwrap();

        assert_eq!(unsubscribe_ack_variable_header.packet_identifier, 0x2211);
    }
}
