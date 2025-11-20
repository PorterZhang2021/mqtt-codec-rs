use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::mqtt_utils;

pub(crate) struct PubRelVariableHeader {
    packet_identifier: u16,
}
impl PubRelVariableHeader {
    fn parse(bytes: &mut impl ByteOperations) -> Result<PubRelVariableHeader, MQTTProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(PubRelVariableHeader { packet_identifier })
    }
}

#[cfg(test)]
mod pub_rel_variable_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;


    use bytes::BytesMut;
    use crate::protocol::mqtt::mqtt4::packet::pub_rel::PubRelVariableHeader;

    #[test]
    fn pub_rel_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0b0010_1010);
        bytes.write_a_byte(0b0010_1010);

        let pub_rel_variable_header = PubRelVariableHeader::parse(&mut bytes).unwrap();

        assert_eq!(
            pub_rel_variable_header.packet_identifier,
            0b0010_1010_0010_1010
        );
    }
}
