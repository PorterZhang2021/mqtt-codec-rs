use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::mqtt_utils;

pub(crate) struct PubRecVariableHeader {
    packet_identifier: u16,
}
impl PubRecVariableHeader {
    fn parse(bytes: &mut impl ByteOperations) -> Result<PubRecVariableHeader, MQTTProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(PubRecVariableHeader { packet_identifier })
    }
}



#[cfg(test)]
mod pub_rec_variable_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use bytes::BytesMut;
    use crate::protocol::mqtt::mqtt4::packet::pub_rec::PubRecVariableHeader;

    #[test]
    fn pub_rec_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0x12);
        bytes.write_a_byte(0x34);

        let pub_rec_variable_header = PubRecVariableHeader::parse(&mut bytes).unwrap();

        assert_eq!(pub_rec_variable_header.packet_identifier, 0x1234);
    }
}