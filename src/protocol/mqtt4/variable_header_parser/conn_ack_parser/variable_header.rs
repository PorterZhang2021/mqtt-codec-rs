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

use crate::protocol::common::return_code::ReturnCode;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub(crate) struct ConnAckVariableHeader {
    session_present: bool,
    return_code: ReturnCode,
}

#[allow(dead_code)]
impl ConnAckVariableHeader {
    pub fn new(session_present: bool, return_code: ReturnCode) -> Self {
        ConnAckVariableHeader {
            session_present,
            return_code,
        }
    }

    pub fn session_present(&self) -> bool {
        self.session_present
    }

    pub fn return_code(&self) -> &ReturnCode {
        &self.return_code
    }
}

#[cfg(test)]
mod conn_ack_variable_header_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::common::return_code::ReturnCode;
    use crate::protocol::mqtt_protocol_error::MqttProtocolError;
    use crate::protocol::mqtt4::variable_header_parser::conn_ack_parser::variable_header::ConnAckVariableHeader;
    use crate::protocol::mqtt4::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderEncoder;
    use bytes::BytesMut;

    #[test]
    fn conn_ack_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        let conn_ack_variable_header =
            ConnAckVariableHeader::new(true, ReturnCode::ConnectionAccepted);
        let en_conn_ack_variable_header = conn_ack_variable_header.encode(vec![]).unwrap();
        bytes.extend_from_slice(&en_conn_ack_variable_header);

        let variable_header = ConnAckVariableHeader::decode(&mut bytes).unwrap();

        assert_eq!(en_conn_ack_variable_header.len(), 2);
        assert_eq!(
            conn_ack_variable_header.return_code(),
            variable_header.return_code()
        );
        assert_eq!(
            conn_ack_variable_header.session_present(),
            variable_header.session_present()
        );
    }

    #[test]
    fn conn_ack_should_failed_when_packet_too_short() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0b0000_0001); // session present = 1
        // missing return code byte

        let result = ConnAckVariableHeader::decode(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MqttProtocolError::PacketTooShort)));
    }

    #[test]
    fn conn_ack_should_failed_when_return_code_invalid() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0b0000_0000); // session present = 0
        bytes.write_a_byte(0x06); // invalid return code

        let result = ConnAckVariableHeader::decode(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MqttProtocolError::ReservedReturnCode)));
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
        assert!(matches!(result, Err(MqttProtocolError::MalformedPacket)))
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
