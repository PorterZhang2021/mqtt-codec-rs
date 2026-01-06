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
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct UnSubscribePayload {
    topics: Vec<String>,
}

#[allow(dead_code)]
impl UnSubscribePayload {
    pub fn new(topics: Vec<String>) -> Self {
        UnSubscribePayload { topics }
    }
    pub fn topics(&self) -> &[String] {
        &self.topics
    }
}

#[cfg(test)]
mod unsubscribe_payload_tests {
    use crate::protocol::mqtt_protocol_error::MqttProtocolError;
    use crate::protocol::mqtt4::payload_parser::mqtt_payload_codec::MqttPayloadEncoder;
    use crate::protocol::mqtt4::payload_parser::unsubscribe_parser::payload::UnSubscribePayload;
    use bytes::BytesMut;

    #[test]
    fn unsubscribe_payload_can_parse_a_topic() {
        let mut bytes = BytesMut::new();
        let topic = "test/topic";

        let topics = vec![topic.to_string()];
        let expect_unsubscribe_payload = UnSubscribePayload::new(topics);
        let encode_expect_unsubscribe_payload = expect_unsubscribe_payload.encode().unwrap();
        bytes.extend_from_slice(&encode_expect_unsubscribe_payload);

        let unsubscribe_payload = UnSubscribePayload::decode(&mut bytes).unwrap();

        assert_eq!(
            unsubscribe_payload.topics().len(),
            expect_unsubscribe_payload.topics().len()
        );
        assert_eq!(
            unsubscribe_payload.topics()[0],
            expect_unsubscribe_payload.topics()[0]
        );
    }

    #[test]
    fn unsubscribe_payload_can_parse_multiple_topics() {
        let mut bytes = BytesMut::new();
        let topic_1 = "topic/one";
        let topic_2 = "topic/two";
        let topics = vec![topic_1.to_string(), topic_2.to_string()];
        let expect_unsubscribe_payload = UnSubscribePayload::new(topics);
        let encode_expect_unsubscribe_payload = expect_unsubscribe_payload.encode().unwrap();
        bytes.extend_from_slice(&encode_expect_unsubscribe_payload);
        let unsubscribe_payload = UnSubscribePayload::decode(&mut bytes).unwrap();
        assert_eq!(
            unsubscribe_payload.topics().len(),
            expect_unsubscribe_payload.topics().len()
        );
        assert_eq!(
            unsubscribe_payload.topics()[0],
            expect_unsubscribe_payload.topics()[0]
        );
        assert_eq!(
            unsubscribe_payload.topics()[1],
            expect_unsubscribe_payload.topics()[1]
        );
    }

    #[test]
    fn unsubscribe_payload_fails_on_empty_packet() {
        let mut bytes = BytesMut::new();
        let result = UnSubscribePayload::decode(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MqttProtocolError::MalformedPacket)));
    }

    #[test]
    fn unsubscribe_payload_can_handle_wildcard_topics() {
        let mut bytes = BytesMut::new();
        let topic_1 = "home/+/temperature";
        let topic_2 = "sensors/#";
        let topics = vec![topic_1.to_string(), topic_2.to_string()];
        let expect_unsubscribe_payload = UnSubscribePayload::new(topics);
        let encoded_expect_unsubscribe_payload = expect_unsubscribe_payload.encode().unwrap();
        bytes.extend_from_slice(&encoded_expect_unsubscribe_payload);

        let unsubscribe_payload = UnSubscribePayload::decode(&mut bytes).unwrap();

        assert_eq!(
            unsubscribe_payload.topics().len(),
            expect_unsubscribe_payload.topics().len()
        );
        assert_eq!(
            unsubscribe_payload.topics()[0],
            expect_unsubscribe_payload.topics()[0]
        );
        assert_eq!(
            unsubscribe_payload.topics()[1],
            expect_unsubscribe_payload.topics()[1]
        );
    }
}
