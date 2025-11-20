use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::mqtt_utils;

pub(crate) struct UnSubScribeVariableHeader {
    packet_identifier: u16,
}
impl UnSubScribeVariableHeader {
    fn parse(
        bytes: &mut impl ByteOperations,
    ) -> Result<UnSubScribeVariableHeader, MQTTProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(UnSubScribeVariableHeader { packet_identifier })
    }
}

#[cfg(test)]
mod unsubscribe_variable_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::mqtt::mqtt4::packet::unsubscribe::UnSubScribeVariableHeader;
    use bytes::BytesMut;

    #[test]
    fn unsubscribe_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0x22);
        bytes.write_a_byte(0x11);

        let unsubscribe_variable_header = UnSubScribeVariableHeader::parse(&mut bytes).unwrap();

        assert_eq!(unsubscribe_variable_header.packet_identifier, 0x2211);
    }
}
