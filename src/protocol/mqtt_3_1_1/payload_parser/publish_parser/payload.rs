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
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct PublishPayload {
    application_message: String,
}

#[allow(dead_code)]
impl PublishPayload {
    pub fn new(application_message: String) -> Self {
        PublishPayload {
            application_message,
        }
    }
    pub fn application_message(&self) -> &str {
        &self.application_message
    }
}

#[cfg(test)]
mod publish_payload_tests {
    use crate::protocol::mqtt_3_1_1::payload_parser::payload_codec::PayloadEncoder;
    use crate::protocol::mqtt_3_1_1::payload_parser::publish_parser::payload::PublishPayload;
    use bytes::BytesMut;

    #[test]
    fn publish_payload_parser_should_parse_payload_correctly() {
        let expect_application_message = "Hello MQTT";

        let publish_payload = PublishPayload::new(expect_application_message.to_string());
        let vec = publish_payload.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&vec);

        let publish_payload = PublishPayload::decode(&mut bytes).unwrap();

        assert_eq!(
            publish_payload.application_message(),
            expect_application_message
        );
    }

    #[test]
    fn publish_payload_can_handle_empty_message() {
        let expect_application_message = "";
        let mut bytes = BytesMut::new();
        let publish_payload = PublishPayload::new(expect_application_message.to_string());
        let vec = publish_payload.encode().unwrap();
        bytes.extend_from_slice(&vec);

        let publish_payload = PublishPayload::decode(&mut bytes).unwrap();

        assert_eq!(
            publish_payload.application_message(),
            expect_application_message
        );
    }

    #[test]
    fn publish_payload_can_not_exceed_max_length() {
        let exceed_message = "A".repeat(70000);
        let payload = PublishPayload::new(exceed_message.to_string());
        let result = payload.encode();
        assert!(result.is_err());
    }
}
