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

use crate::protocol::common::qos::QoSCode;
#[allow(dead_code)]
pub(crate) struct SubscribePayload {
    subscription_and_qos_tuples: Vec<(String, QoSCode)>,
}

#[allow(dead_code)]
impl SubscribePayload {
    pub fn new(subscription_and_qos_tuples: Vec<(String, QoSCode)>) -> Self {
        SubscribePayload {
            subscription_and_qos_tuples,
        }
    }
    pub fn subscription_and_qos_tuples(&self) -> &[(String, QoSCode)] {
        &self.subscription_and_qos_tuples
    }
}

#[cfg(test)]
mod subscribe_payload_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::common::qos::QoSCode;
    use crate::protocol::mqtt_protocol_error::MqttProtocolError;
    use crate::protocol::mqtt4::payload_parser::mqtt_payload_codec::MqttPayloadEncoder;
    use crate::protocol::mqtt4::payload_parser::subscribe_parser::payload::SubscribePayload;
    use crate::utils::utf::utf_8_handler::write;
    use bytes::BytesMut;

    #[test]
    fn subscribe_payload_can_parse_a_topic() {
        let mut bytes = BytesMut::new();
        let topic_filter = "test/topic";
        let topic_qos = 0b0000_0001;
        let topic_vec = vec![(topic_filter.to_string(), QoSCode::parse(topic_qos).unwrap())];
        let expect_subscribe_payload = SubscribePayload::new(topic_vec);
        let encode_expect_subscribe_payload = expect_subscribe_payload.encode().unwrap();
        bytes.extend(encode_expect_subscribe_payload);

        let subscribe_payload = SubscribePayload::decode(&mut bytes).unwrap();

        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples().len(),
            expect_subscribe_payload.subscription_and_qos_tuples().len()
        );
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples()[0].0,
            expect_subscribe_payload.subscription_and_qos_tuples()[0].0
        );
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples()[0].1,
            expect_subscribe_payload.subscription_and_qos_tuples()[0].1
        );
    }

    #[test]
    fn subscribe_payload_can_parse_multiple_topics() {
        let mut bytes = BytesMut::new();

        let topic_filter_1 = "topic/one";
        let topic_qos_1 = QoSCode::Qos0;
        let topic_filter_2 = "topic/two";
        let topic_qos_2 = QoSCode::Qos2;
        let topic_vec = vec![
            (topic_filter_1.to_string(), topic_qos_1),
            (topic_filter_2.to_string(), topic_qos_2),
        ];

        let expect_subscribe_payload = SubscribePayload::new(topic_vec);

        let encode_expect_subscribe_payload = expect_subscribe_payload.encode().unwrap();
        bytes.extend(encode_expect_subscribe_payload);

        let subscribe_payload = SubscribePayload::decode(&mut bytes).unwrap();
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples().len(),
            expect_subscribe_payload.subscription_and_qos_tuples().len()
        );
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples()[0].0,
            expect_subscribe_payload.subscription_and_qos_tuples()[0].0
        );
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples()[0].1,
            expect_subscribe_payload.subscription_and_qos_tuples()[0].1
        );
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples()[1].0,
            expect_subscribe_payload.subscription_and_qos_tuples()[1].0
        );
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples()[1].1,
            expect_subscribe_payload.subscription_and_qos_tuples()[1].1
        );
    }
    #[test]
    fn subscribe_payload_fails_on_invalid_qos() {
        let mut bytes = BytesMut::new();
        let _ = write(&mut bytes, "invalid/qos");
        bytes.write_a_byte(0b0000_0011); // Invalid QoS
        let result = SubscribePayload::decode(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MqttProtocolError::InvalidQoS(3))));
    }

    #[test]
    fn subscribe_payload_fails_on_incomplete_topic() {
        let mut bytes = BytesMut::new();
        let _ = write(&mut bytes, "incomplete/topic");
        // Missing QoS byte
        let result = SubscribePayload::decode(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MqttProtocolError::PacketTooShort)));
    }

    #[test]
    fn subscribe_payload_fails_on_empty_packet() {
        let mut bytes = BytesMut::new();
        let result = SubscribePayload::decode(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MqttProtocolError::MalformedPacket)));
    }

    #[test]
    fn subscribe_payload_can_handle_wildcard_topics() {
        let mut bytes = BytesMut::new();
        let topic_filter_1 = "home/+/temperature";
        let topic_filter_qos_1 = 0b0000_0001;
        let topic_filter_2 = "sensors/#";
        let topic_filter_qos_2 = 0b0000_0000;

        let topic_vec = vec![
            (
                topic_filter_1.to_string(),
                QoSCode::parse(topic_filter_qos_1).unwrap(),
            ),
            (
                topic_filter_2.to_string(),
                QoSCode::parse(topic_filter_qos_2).unwrap(),
            ),
        ];

        let expect_subscribe_payload = SubscribePayload::new(topic_vec);
        let encode_expect_subscribe_payload = expect_subscribe_payload.encode().unwrap();
        bytes.extend(encode_expect_subscribe_payload);

        let subscribe_payload = SubscribePayload::decode(&mut bytes).unwrap();
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples().len(),
            expect_subscribe_payload.subscription_and_qos_tuples().len()
        );
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples()[0].0,
            expect_subscribe_payload.subscription_and_qos_tuples()[0].0
        );
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples()[0].1,
            expect_subscribe_payload.subscription_and_qos_tuples()[0].1
        );
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples()[1].0,
            expect_subscribe_payload.subscription_and_qos_tuples()[1].0
        );
        assert_eq!(
            subscribe_payload.subscription_and_qos_tuples()[1].1,
            expect_subscribe_payload.subscription_and_qos_tuples()[1].1
        );
    }
}
