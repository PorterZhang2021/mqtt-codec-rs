// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::byte_adapter::byte_operations::ByteOperations;
use crate::protocol::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt4::mqtt_codec::MqttVariableHeaderCodec;
use crate::protocol::mqtt4::return_code::ReturnCode;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub(crate) struct ConnAckVariableHeader {
    pub(crate) session_present: bool,
    pub(crate) return_code: ReturnCode,
}

#[allow(dead_code)]
impl ConnAckVariableHeader {
    pub(crate) fn new(session_present: bool, return_code: ReturnCode) -> Self {
        ConnAckVariableHeader {
            session_present,
            return_code,
        }
    }

    pub(crate) fn return_code(&self) -> &ReturnCode {
        &self.return_code
    }
}

impl MqttVariableHeaderCodec for ConnAckVariableHeader {
    fn decode(
        _fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnAckVariableHeader, MQTTProtocolError> {
        Self::parse(bytes)
    }

    fn encode(_variable_header: ConnAckVariableHeader) -> Result<&'static [u8], MQTTProtocolError> {
        todo!()
    }
}

#[allow(dead_code)]
impl ConnAckVariableHeader {
    fn parse(bytes: &mut impl ByteOperations) -> Result<ConnAckVariableHeader, MQTTProtocolError> {
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
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::mqtt_protocol_error::MQTTProtocolError;
    use crate::protocol::mqtt4::variable_header_parser::conn_ack::ConnAckVariableHeader;
    use bytes::BytesMut;

    #[test]
    fn conn_ack_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0b0000_0001); // session present = 1
        bytes.write_a_byte(0x00); // return code = Connection Accepted

        let variable_header = ConnAckVariableHeader::parse(&mut bytes).unwrap();

        assert!(variable_header.session_present);
        assert!(matches!(
            variable_header.return_code,
            crate::protocol::mqtt4::return_code::ReturnCode::ConnectionAccepted
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
