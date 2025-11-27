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

use crate::byte_wrapper::byte_operations::ByteOperations;
use crate::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::utils::utf;

#[allow(dead_code)]
struct SubscribePayload {
    topics: Vec<(String, u8)>,
}

#[allow(dead_code)]
impl SubscribePayload {
    pub(crate) fn parse(
        bytes: &mut impl ByteOperations,
    ) -> Result<SubscribePayload, MQTTProtocolError> {
        let mut topics = Vec::new();

        while let Some(topic) = Self::parse_topic_with_qos(bytes)? {
            topics.push(topic);
        }

        Self::verify_topics_is_empty(&mut topics)?;

        Ok(SubscribePayload { topics })
    }

    fn parse_topic_with_qos(
        bytes: &mut impl ByteOperations,
    ) -> Result<Option<(String, u8)>, MQTTProtocolError> {
        if bytes.is_empty() {
            return Ok(None);
        }

        let topic_filter = Self::parse_topic_filter(bytes)?;
        let qos = Self::parse_qos(bytes)?;

        Ok(Some((topic_filter, qos)))
    }

    fn parse_topic_filter(bytes: &mut impl ByteOperations) -> Result<String, MQTTProtocolError> {
        let topic_filter = utf::utf_8_handler::read(bytes)?;
        Ok(topic_filter)
    }

    fn parse_qos(bytes: &mut impl ByteOperations) -> Result<u8, MQTTProtocolError> {
        let qos = bytes
            .read_a_byte()
            .ok_or(MQTTProtocolError::PacketTooShort)?;
        Self::verify_qos_is_exceed_three(qos)?;
        Ok(qos)
    }

    fn verify_qos_is_exceed_three(qos_byte: u8) -> Result<(), MQTTProtocolError> {
        if qos_byte > 2 {
            return Err(MQTTProtocolError::MalformedPacket);
        }
        Ok(())
    }

    fn verify_topics_is_empty(topics: &mut [(String, u8)]) -> Result<(), MQTTProtocolError> {
        if topics.is_empty() {
            return Err(MQTTProtocolError::MalformedPacket);
        }
        Ok(())
    }
}

#[cfg(test)]
mod subscribe_payload_tests {
    use crate::byte_wrapper::byte_operations::ByteOperations;
    use crate::mqtt::mqtt_protocol_error::MQTTProtocolError;
    use crate::mqtt::mqtt4::payload_parser::subscribe::SubscribePayload;
    use crate::utils::utf::utf_8_handler::write;
    use bytes::BytesMut;

    #[test]
    fn subscribe_payload_can_parse_a_topic() {
        let mut bytes = BytesMut::new();
        let _ = write(&mut bytes, "test/topic");
        bytes.write_a_byte(0b0000_0001);

        let subscribe_payload = SubscribePayload::parse(&mut bytes).unwrap();

        assert_eq!(subscribe_payload.topics.len(), 1);
        assert_eq!(subscribe_payload.topics[0].0, "test/topic");
        assert_eq!(subscribe_payload.topics[0].1, 1);
    }

    #[test]
    fn subscribe_payload_can_parse_multiple_topics() {
        let mut bytes = BytesMut::new();
        let _ = write(&mut bytes, "topic/one");
        bytes.write_a_byte(0b0000_0000);
        let _ = write(&mut bytes, "topic/two");
        bytes.write_a_byte(0b0000_0010);
        let subscribe_payload = SubscribePayload::parse(&mut bytes).unwrap();
        assert_eq!(subscribe_payload.topics.len(), 2);
        assert_eq!(subscribe_payload.topics[0].0, "topic/one");
        assert_eq!(subscribe_payload.topics[0].1, 0);
        assert_eq!(subscribe_payload.topics[1].0, "topic/two");
        assert_eq!(subscribe_payload.topics[1].1, 2);
    }
    #[test]
    fn subscribe_payload_fails_on_invalid_qos() {
        let mut bytes = BytesMut::new();
        let _ = write(&mut bytes, "invalid/qos");
        bytes.write_a_byte(0b0000_0011); // Invalid QoS
        let result = SubscribePayload::parse(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MQTTProtocolError::MalformedPacket)));
    }

    #[test]
    fn subscribe_payload_fails_on_incomplete_topic() {
        let mut bytes = BytesMut::new();
        let _ = write(&mut bytes, "incomplete/topic");
        // Missing QoS byte
        let result = SubscribePayload::parse(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MQTTProtocolError::PacketTooShort)));
    }

    #[test]
    fn subscribe_payload_fails_on_empty_packet() {
        let mut bytes = BytesMut::new();
        let result = SubscribePayload::parse(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MQTTProtocolError::MalformedPacket)));
    }

    #[test]
    fn subscribe_payload_can_handle_wildcard_topics() {
        let mut bytes = BytesMut::new();
        let _ = write(&mut bytes, "home/+/temperature");
        bytes.write_a_byte(0b0000_0001);
        let _ = write(&mut bytes, "sensors/#");
        bytes.write_a_byte(0b0000_0000);
        let subscribe_payload = SubscribePayload::parse(&mut bytes).unwrap();
        assert_eq!(subscribe_payload.topics.len(), 2);
        assert_eq!(subscribe_payload.topics[0].0, "home/+/temperature");
        assert_eq!(subscribe_payload.topics[0].1, 1);
        assert_eq!(subscribe_payload.topics[1].0, "sensors/#");
        assert_eq!(subscribe_payload.topics[1].1, 0);
    }
}
