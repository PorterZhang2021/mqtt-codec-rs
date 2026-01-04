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
use crate::protocol::common::qos::QoSCode;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt4::fixed_header_parser::fixed_header_flags::FixedHeaderFlags;
use crate::protocol::mqtt4::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderDecoder;
use crate::protocol::mqtt4::variable_header_parser::publish_parser::variable_header::PublishVariableHeader;
use crate::utils::radix::radix_handler;
use crate::utils::utf::utf_8_handler;

impl MqttVariableHeaderDecoder for PublishVariableHeader {
    fn decode(
        fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<PublishVariableHeader, MqttProtocolError> {
        if let FixedHeaderFlags::Publish { qos, .. } = fixed_header.fixed_header_reserved_flags() {
            PublishVariableHeader::decode(bytes, qos)
        } else {
            Err(MqttProtocolError::MalformedPacket)
        }
    }
}

#[allow(dead_code)]
impl PublishVariableHeader {
    pub(super) fn decode(
        bytes: &mut impl ByteOperations,
        qos_level: &QoSCode,
    ) -> Result<PublishVariableHeader, MqttProtocolError> {
        let topic_name = Self::parse_topic_name(bytes)?;
        let packet_identifier = Self::parse_packet_identifier(bytes, qos_level)?;
        Ok(PublishVariableHeader::new(topic_name, packet_identifier))
    }
    pub(super) fn parse_topic_name(
        bytes: &mut impl ByteOperations,
    ) -> Result<String, MqttProtocolError> {
        let non_verify_topic_name = utf_8_handler::read(bytes)?;
        Self::verify_topic_name(&non_verify_topic_name)?;
        Ok(non_verify_topic_name)
    }

    pub(super) fn verify_topic_name(topic_name: &str) -> Result<(), MqttProtocolError> {
        if topic_name.is_empty() {
            return Err(MqttProtocolError::MalformedPacket);
        }
        if topic_name.contains('#') || topic_name.contains('+') {
            return Err(MqttProtocolError::MalformedPacket);
        }
        Ok(())
    }

    pub(super) fn parse_packet_identifier(
        bytes: &mut impl ByteOperations,
        qos_level: &QoSCode,
    ) -> Result<Option<u16>, MqttProtocolError> {
        match qos_level {
            QoSCode::Qos0 => Ok(None),
            QoSCode::Qos1 | QoSCode::Qos2 => {
                let bytes = bytes.read_bytes(2);
                let packet_identifier = radix_handler::be_bytes_to_u16(bytes.as_slice())?;
                Ok(Some(packet_identifier))
            }
        }
    }
}
