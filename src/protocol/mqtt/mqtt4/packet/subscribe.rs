use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::mqtt_utils;

pub(crate) struct SubScribeVariableHeader {
    packet_identifier: u16,
}
impl SubScribeVariableHeader {
    fn parse(bytes: &mut impl ByteOperations) -> Result<SubScribeVariableHeader, MQTTProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(SubScribeVariableHeader { packet_identifier })
    }
}



#[cfg(test)]
mod subscribe_variable_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use bytes::BytesMut;
    use crate::protocol::mqtt::mqtt4::packet::subscribe::SubScribeVariableHeader;

    #[test]
    fn subscribe_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0x21);
        bytes.write_a_byte(0x31);

        let subscribe_variable_header = SubScribeVariableHeader::parse(&mut bytes).unwrap();

        assert_eq!(subscribe_variable_header.packet_identifier, 0x2131);
    }
}