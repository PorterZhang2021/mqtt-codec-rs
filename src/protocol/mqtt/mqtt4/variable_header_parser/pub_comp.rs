use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::mqtt_utils;

pub(crate) struct PubCompVariableHeader {
    packet_identifier: u16,
}
impl PubCompVariableHeader {
    fn parse(bytes: &mut impl ByteOperations) -> Result<PubCompVariableHeader, MQTTProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(PubCompVariableHeader { packet_identifier })
    }
}

#[cfg(test)]
mod pub_comp_variable_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;

    use crate::protocol::mqtt::mqtt4::variable_header_parser::pub_comp::PubCompVariableHeader;
    use bytes::BytesMut;

    #[test]
    fn pub_comp_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0b0000_1010);
        bytes.write_a_byte(0b0010_1010);

        let pub_comp_variable_header = PubCompVariableHeader::parse(&mut bytes).unwrap();

        assert_eq!(
            pub_comp_variable_header.packet_identifier,
            0b0000_1010_0010_1010
        );
    }
}
