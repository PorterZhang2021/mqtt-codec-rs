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
use crate::protocol::common::protocol_level::ProtocolLevel;
use crate::protocol::common::qos::QoSCode;
use crate::protocol::mqtt_3_1_1::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt_3_1_1::variable_header_parser::connect_parser::variable_header::{
    ConnectFlags, ConnectVariableHeader,
};
use crate::protocol::mqtt_3_1_1::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderDecoder;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::utils::radix::radix_handler;
use crate::utils::utf::utf_8_handler;

impl MqttVariableHeaderDecoder for ConnectVariableHeader {
    fn decode(
        _fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnectVariableHeader, MqttProtocolError> {
        Self::decode(bytes)
    }
}

#[allow(dead_code)]
impl ConnectVariableHeader {
    pub(super) fn decode(
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnectVariableHeader, MqttProtocolError> {
        Self::verify_protocol_name(bytes)?;
        let protocol_level = Self::verify_and_return_protocol_level(bytes)?;
        let connect_flags = Self::parser_connect_flags(bytes)?;
        let keep_alive = Self::parse_keep_alive(bytes)?;

        Ok(ConnectVariableHeader::new(
            protocol_level,
            connect_flags,
            keep_alive,
        ))
    }
    pub(super) fn verify_protocol_name(
        bytes: &mut impl ByteOperations,
    ) -> Result<(), MqttProtocolError> {
        let protocol_name = utf_8_handler::read(bytes)?;
        if protocol_name != "MQTT" {
            return Err(MqttProtocolError::ProtocolNameError(protocol_name));
        }
        Ok(())
    }
    pub(super) fn verify_and_return_protocol_level(
        bytes: &mut impl ByteOperations,
    ) -> Result<ProtocolLevel, MqttProtocolError> {
        let protocol_level = bytes
            .read_a_byte()
            .ok_or(MqttProtocolError::PacketTooShort)?;
        ProtocolLevel::parse(protocol_level)
    }

    pub(super) fn parser_connect_flags(
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnectFlags, MqttProtocolError> {
        let connect_flags_byte = bytes
            .read_a_byte()
            .ok_or(MqttProtocolError::PacketTooShort)?;

        let user_name_flag = ConnectVariableHeader::parse_user_name_flag(connect_flags_byte);
        let password_flag = ConnectVariableHeader::parse_password_flag(connect_flags_byte);
        let will_retain = ConnectVariableHeader::parse_will_retain(connect_flags_byte);
        let will_qos = ConnectVariableHeader::parse_qos(connect_flags_byte)?;
        let will_flag = ConnectVariableHeader::parse_will_flag(connect_flags_byte);
        let clean_session = ConnectVariableHeader::parse_clean_session(connect_flags_byte);
        ConnectVariableHeader::verify_reserved_bit(connect_flags_byte)?;

        let connect_flags = ConnectFlags::new(
            user_name_flag,
            password_flag,
            will_retain,
            will_qos,
            will_flag,
            clean_session,
        )?;

        Ok(connect_flags)
    }

    pub(super) fn parse_user_name_flag(connect_flags_byte: u8) -> bool {
        (connect_flags_byte & 0b1000_0000) != 0
    }

    pub(super) fn parse_password_flag(connect_flags_byte: u8) -> bool {
        (connect_flags_byte & 0b0100_0000) != 0
    }

    pub(super) fn parse_will_retain(connect_flags_byte: u8) -> bool {
        (connect_flags_byte & 0b0010_0000) != 0
    }

    pub(super) fn parse_qos(connect_flags_byte: u8) -> Result<QoSCode, MqttProtocolError> {
        let value = (connect_flags_byte & 0b0001_1000) >> 3;
        QoSCode::parse(value)
    }

    pub(super) fn parse_will_flag(connect_flags_byte: u8) -> bool {
        (connect_flags_byte & 0b0000_0100) != 0
    }

    pub(super) fn parse_clean_session(connect_flags_byte: u8) -> bool {
        (connect_flags_byte & 0b0000_0010) != 0
    }

    pub(super) fn verify_reserved_bit(connect_flags_byte: u8) -> Result<(), MqttProtocolError> {
        let reserved_bit = connect_flags_byte & 0b0000_0001;
        if reserved_bit != 0 {
            return Err(MqttProtocolError::MalformedPacket);
        }
        Ok(())
    }

    pub(super) fn parse_keep_alive(
        bytes: &mut impl ByteOperations,
    ) -> Result<u16, MqttProtocolError> {
        let length_bytes = bytes.read_bytes(2);
        let keep_alive = radix_handler::be_bytes_to_u16(length_bytes.as_slice())?;
        Ok(keep_alive)
    }
}
