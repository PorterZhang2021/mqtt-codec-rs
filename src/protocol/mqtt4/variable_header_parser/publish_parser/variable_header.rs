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

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub(crate) struct PublishVariableHeader {
    topic_name: String,
    packet_identifier: Option<u16>,
}

#[allow(dead_code)]
impl PublishVariableHeader {
    pub fn new(topic_name: String, packet_identifier: Option<u16>) -> Self {
        PublishVariableHeader {
            topic_name,
            packet_identifier,
        }
    }

    pub fn topic_name(&self) -> &str {
        &self.topic_name
    }

    pub fn packet_identifier(&self) -> Option<u16> {
        self.packet_identifier
    }
}

#[cfg(test)]
mod publish_variable_header_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::common::qos::QoSCode;
    use crate::protocol::mqtt_protocol_error::MqttProtocolError;
    use crate::protocol::mqtt4::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderEncoder;
    use crate::protocol::mqtt4::variable_header_parser::publish_parser::variable_header::PublishVariableHeader;
    use crate::utils::utf::utf_8_handler::write;
    use bytes::BytesMut;

    #[test]
    fn publish_variable_header_can_encode_and_decode() {
        let original_header = PublishVariableHeader::new("test/topic".to_string(), Some(0x1234));
        let encoded_bytes = original_header.encode(vec![]).unwrap();

        let mut bytes_mut = BytesMut::from(&encoded_bytes[..]);
        let decoded_header = PublishVariableHeader::decode(&mut bytes_mut, &QoSCode::Qos1).unwrap();

        assert_eq!(original_header, decoded_header);
    }

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
            MqttProtocolError::MalformedPacket
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
            MqttProtocolError::MalformedPacket
        ));

        let mut bytes_mut2 = BytesMut::new();
        write(&mut bytes_mut2, "a/b/#").unwrap();

        let result2 = PublishVariableHeader::parse_topic_name(&mut bytes_mut2);

        assert!(result2.is_err());
        assert!(matches!(
            result2.err().unwrap(),
            MqttProtocolError::MalformedPacket
        ));
    }

    #[test]
    fn publish_variable_header_qos_0_no_packet_identifier() {
        let mut bytes_mut = BytesMut::new();
        let result =
            PublishVariableHeader::parse_packet_identifier(&mut bytes_mut, &QoSCode::Qos0).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn publish_variable_header_qos_1_packet_identifier() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_a_byte(0x12);
        bytes_mut.write_a_byte(0x34);
        let result =
            PublishVariableHeader::parse_packet_identifier(&mut bytes_mut, &QoSCode::Qos1).unwrap();
        assert_eq!(result, Some(0x1234));
    }
}
