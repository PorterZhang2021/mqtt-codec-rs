use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::mqtt::mqtt4::return_code::ReturnCode;
use bytes::Bytes;

pub(crate) struct ConnAckVariableHeader {
    pub session_present: bool,
    pub return_code: ReturnCode,
}

impl ConnAckVariableHeader {
    pub(crate) fn parse(
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnAckVariableHeader, MQTTProtocolError> {
        let session_present = Self::verify_reserved_byte_and_parse_session_present_flag(bytes)?;

        let return_code = Self::parse_return_code(bytes)?;

        Ok(ConnAckVariableHeader {
            session_present,
            return_code,
        })
    }

    fn verify_reserved_byte_and_parse_session_present_flag(
        bytes: &mut impl ByteOperations,
    ) -> Result<bool, MQTTProtocolError> {
        let reserved_byte = bytes
            .read_a_byte()
            .ok_or(MQTTProtocolError::PacketTooShort)?;

        Self::verify_reserved_bits(reserved_byte)?;

        let session_present = Self::parse_session_present_flag(reserved_byte);

        Ok(session_present)
    }

    fn verify_reserved_bits(reserved_byte: u8) -> Result<(), MQTTProtocolError> {
        if (reserved_byte & 0b1111_1110) != 0 {
            return Err(MQTTProtocolError::MalformedPacket);
        }
        Ok(())
    }

    fn parse_session_present_flag(reserved_byte: u8) -> bool {
        (reserved_byte & 0b0000_0001) == 1
    }

    fn parse_return_code(bytes: &mut impl ByteOperations) -> Result<ReturnCode, MQTTProtocolError> {
        let return_code_byte = bytes
            .read_a_byte()
            .ok_or(MQTTProtocolError::PacketTooShort)?;
        ReturnCode::parse(return_code_byte)
    }
}

#[cfg(test)]
mod conn_ack_variable_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
    use crate::protocol::mqtt::mqtt4::packet::conn_ack::ConnAckVariableHeader;
    use bytes::BytesMut;
    use std::io::Bytes;

    #[test]
    fn conn_ack_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0b0000_0001); // session present = 1
        bytes.write_a_byte(0x00); // return code = Connection Accepted

        let variable_header = ConnAckVariableHeader::parse(&mut bytes).unwrap();

        assert!(variable_header.session_present);
        assert!(matches!(
            variable_header.return_code,
            crate::protocol::mqtt::mqtt4::return_code::ReturnCode::ConnectionAccepted
        ));
    }

    #[test]
    fn conn_ack_should_failed_when_packet_too_short() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0b0000_0001); // session present = 1
        // missing return code byte

        let result = ConnAckVariableHeader::parse(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MQTTProtocolError::PacketTooShort)));
    }

    #[test]
    fn conn_ack_should_failed_when_return_code_invalid() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0b0000_0000); // session present = 0
        bytes.write_a_byte(0x06); // invalid return code

        let result = ConnAckVariableHeader::parse(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MQTTProtocolError::ReservedReturnCode)));
    }

    #[test]
    fn conn_ack_should_verify_reserved_bits_when_reserved_bits_is_ok() {
        let reserved_byte: u8 = 0b0000_0001;

        let result = ConnAckVariableHeader::verify_reserved_bits(reserved_byte);
        assert!(result.is_ok());
    }
    #[test]
    fn conn_ack_should_verify_reserved_bits_when_reserved_bits_is_error() {
        let reserved_byte: u8 = 0b0000_0010;
        let result = ConnAckVariableHeader::verify_reserved_bits(reserved_byte);
        assert!(result.is_err());
        assert!(matches!(result, Err(MQTTProtocolError::MalformedPacket)))
    }

    #[test]
    fn conn_ack_should_parse_session_present_flag_when_flag_set_1() {
        let reserved_byte: u8 = 0b0000_0001;
        let is_session_present = ConnAckVariableHeader::parse_session_present_flag(reserved_byte);
        assert!(is_session_present);
    }

    #[test]
    fn conn_ack_should_parse_session_present_flag_when_flag_set_0() {
        let reserved_byte: u8 = 0b0000_0000;
        let is_session_present = ConnAckVariableHeader::parse_session_present_flag(reserved_byte);
        assert!(!is_session_present);
    }
}
