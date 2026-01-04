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
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt4::return_code::ReturnCode;
use crate::protocol::mqtt4::variable_header_parser::conn_ack_parser::variable_header::ConnAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderDecoder;

impl MqttVariableHeaderDecoder for ConnAckVariableHeader {
    fn decode(
        _fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnAckVariableHeader, MqttProtocolError> {
        Self::decode(bytes)
    }
}

#[allow(dead_code)]
impl ConnAckVariableHeader {
    pub(super) fn decode(
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnAckVariableHeader, MqttProtocolError> {
        let session_present = Self::verify_reserved_byte_and_parse_session_present_flag(bytes)?;

        let return_code = Self::parse_return_code(bytes)?;

        Ok(ConnAckVariableHeader::new(session_present, return_code))
    }
    fn verify_reserved_byte_and_parse_session_present_flag(
        bytes: &mut impl ByteOperations,
    ) -> Result<bool, MqttProtocolError> {
        let reserved_byte = bytes
            .read_a_byte()
            .ok_or(MqttProtocolError::PacketTooShort)?;

        Self::verify_reserved_bits(reserved_byte)?;

        let session_present = Self::parse_session_present_flag(reserved_byte);

        Ok(session_present)
    }

    pub(super) fn verify_reserved_bits(reserved_byte: u8) -> Result<(), MqttProtocolError> {
        if (reserved_byte & 0b1111_1110) != 0 {
            return Err(MqttProtocolError::MalformedPacket);
        }
        Ok(())
    }

    pub(super) fn parse_session_present_flag(reserved_byte: u8) -> bool {
        (reserved_byte & 0b0000_0001) == 1
    }

    pub(super) fn parse_return_code(
        bytes: &mut impl ByteOperations,
    ) -> Result<ReturnCode, MqttProtocolError> {
        let return_code_byte = bytes
            .read_a_byte()
            .ok_or(MqttProtocolError::PacketTooShort)?;
        ReturnCode::parse(return_code_byte)
    }
}
