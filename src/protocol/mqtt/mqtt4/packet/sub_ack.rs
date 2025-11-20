use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::mqtt_utils;

pub(crate) struct SubAckVariableHeader {
    packet_identifier: u16,
}
impl SubAckVariableHeader {
    fn parse(
        bytes: &mut impl ByteOperations,
    ) -> Result<SubAckVariableHeader, MQTTProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(SubAckVariableHeader { packet_identifier })
    }
}

#[cfg(test)]
mod sub_ack_variable_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use bytes::BytesMut;
    use crate::protocol::mqtt::mqtt4::packet::sub_ack::SubAckVariableHeader;

    #[test]
    fn sub_ack_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0x22);
        bytes.write_a_byte(0x11);

        let sub_ack_variable_header = SubAckVariableHeader::parse(&mut bytes).unwrap();

        assert_eq!(sub_ack_variable_header.packet_identifier, 0x2211);
    }
}
