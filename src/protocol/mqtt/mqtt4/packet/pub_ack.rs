use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::mqtt_utils;
use crate::protocol::utils::radix::radix_handler;

pub(crate) struct PubAckVariableHeader {
    packet_identifier: u16,
}
impl PubAckVariableHeader {
    fn parse(bytes: &mut impl ByteOperations) -> Result<PubAckVariableHeader, MQTTProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(PubAckVariableHeader { packet_identifier })
    }
}

#[cfg(test)]
mod pub_ack_variable_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::mqtt::mqtt4::packet::pub_ack::PubAckVariableHeader;
    use bytes::BytesMut;

    #[test]
    fn pub_ack_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0x00);
        bytes.write_a_byte(0x0A);

        let pub_ack_variable_header = PubAckVariableHeader::parse(&mut bytes).unwrap();

        assert_eq!(pub_ack_variable_header.packet_identifier, 10);
    }
}
