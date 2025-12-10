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
use crate::protocol::mqtt4::fixed_header_parser::fixed_header_flags::FixedHeaderFlags;
use crate::protocol::mqtt4::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderCodec;
use crate::utils::radix::radix_handler;
use crate::utils::utf::utf_8_handler;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub(crate) struct PublishVariableHeader {
    topic_name: String,
    packet_identifier: Option<u16>,
}

#[allow(dead_code)]
impl PublishVariableHeader {
    pub(crate) fn new(topic_name: String, packet_identifier: Option<u16>) -> Self {
        PublishVariableHeader {
            topic_name,
            packet_identifier,
        }
    }
}

impl MqttVariableHeaderCodec for PublishVariableHeader {
    fn decode(
        fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<PublishVariableHeader, MQTTProtocolError> {
        if let FixedHeaderFlags::Publish { qos, .. } = fixed_header.fixed_header_reserved_flags() {
            PublishVariableHeader::parse(bytes, *qos)
        } else {
            Err(MQTTProtocolError::MalformedPacket)
        }
    }

    fn encode(_variable_header: PublishVariableHeader) -> Result<&'static [u8], MQTTProtocolError> {
        todo!()
    }
}

#[allow(dead_code)]
impl PublishVariableHeader {
    fn parse(
        bytes: &mut impl ByteOperations,
        qos_level: u8,
    ) -> Result<PublishVariableHeader, MQTTProtocolError> {
        let topic_name = Self::parse_topic_name(bytes)?;
        let packet_identifier = Self::parse_packet_identifier(bytes, qos_level)?;
        Ok(PublishVariableHeader {
            topic_name,
            packet_identifier,
        })
    }
    fn parse_topic_name(bytes: &mut impl ByteOperations) -> Result<String, MQTTProtocolError> {
        let non_verify_topic_name = utf_8_handler::read(bytes)?;
        Self::verify_topic_name(&non_verify_topic_name)?;
        Ok(non_verify_topic_name)
    }

    fn verify_topic_name(topic_name: &str) -> Result<(), MQTTProtocolError> {
        if topic_name.is_empty() {
            return Err(MQTTProtocolError::MalformedPacket);
        }
        if topic_name.contains('#') || topic_name.contains('+') {
            return Err(MQTTProtocolError::MalformedPacket);
        }
        Ok(())
    }

    fn parse_packet_identifier(
        bytes: &mut impl ByteOperations,
        qos_level: u8,
    ) -> Result<Option<u16>, MQTTProtocolError> {
        match qos_level {
            0 => Ok(None),
            1 | 2 => {
                let bytes = bytes.read_bytes(2);
                let packet_identifier = radix_handler::be_bytes_to_u16(bytes.as_slice())?;
                Ok(Some(packet_identifier))
            }
            _ => Err(MQTTProtocolError::MalformedPacket),
        }
    }
}

#[cfg(test)]
mod publish_variable_header_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::mqtt_protocol_error::MQTTProtocolError;
    use crate::protocol::mqtt4::variable_header_parser::publish::PublishVariableHeader;
    use crate::utils::utf::utf_8_handler::write;
    use bytes::BytesMut;

    #[test]
    fn publish_variable_header_must_contain_topic_name() {
        let mut bytes_mut = BytesMut::new();
        write(&mut bytes_mut, "a/b").unwrap();

        let result = PublishVariableHeader::parse_topic_name(&mut bytes_mut).unwrap();

        assert_eq!(result, "a/b");
    }

    #[test]
    fn publish_variable_header_must_fail_on_empty_topic_name() {
        let mut bytes_mut = BytesMut::new();
        write(&mut bytes_mut, "").unwrap();

        let result = PublishVariableHeader::parse_topic_name(&mut bytes_mut);

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            MQTTProtocolError::MalformedPacket
        ))
    }

    #[test]
    fn publish_variable_header_must_fail_on_topic_name_with_wildcard_characters() {
        let mut bytes_mut = BytesMut::new();
        write(&mut bytes_mut, "a/+/c").unwrap();

        let result = PublishVariableHeader::parse_topic_name(&mut bytes_mut);

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            MQTTProtocolError::MalformedPacket
        ));

        let mut bytes_mut2 = BytesMut::new();
        write(&mut bytes_mut2, "a/b/#").unwrap();

        let result2 = PublishVariableHeader::parse_topic_name(&mut bytes_mut2);

        assert!(result2.is_err());
        assert!(matches!(
            result2.err().unwrap(),
            MQTTProtocolError::MalformedPacket
        ));
    }

    #[test]
    fn publish_variable_header_qos_0_no_packet_identifier() {
        let mut bytes_mut = BytesMut::new();
        let result = PublishVariableHeader::parse_packet_identifier(&mut bytes_mut, 0).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn publish_variable_header_qos_1_packet_identifier() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_a_byte(0x12);
        bytes_mut.write_a_byte(0x34);
        let result = PublishVariableHeader::parse_packet_identifier(&mut bytes_mut, 1).unwrap();
        assert_eq!(result, Some(0x1234));
    }
}
