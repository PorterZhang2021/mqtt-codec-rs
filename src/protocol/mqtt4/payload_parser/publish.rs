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
use crate::protocol::mqtt4::payload_parser::mqtt_payload_codec::MqttPayloadCodec;
use crate::protocol::mqtt4::variable_header_parser::publish::PublishVariableHeader;
use crate::utils::utf;
#[allow(dead_code)]
pub(crate) struct PublishPayload {
    application_message: String,
}
#[allow(dead_code)]
impl PublishPayload {
    pub fn application_message(&self) -> &str {
        &self.application_message
    }
}

impl MqttPayloadCodec<PublishVariableHeader> for PublishPayload {
    fn decode(
        _fixed_header: &FixedHeader,
        _variable_header: &PublishVariableHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<PublishPayload, MQTTProtocolError> {
        Self::parse(bytes)
    }

    fn encode(_payload: Self) -> Result<&'static [u8], MQTTProtocolError> {
        todo!()
    }
}

#[allow(dead_code)]
impl PublishPayload {
    fn parse(bytes: &mut impl ByteOperations) -> Result<PublishPayload, MQTTProtocolError> {
        let application_message = Self::parse_application_message(bytes)?;
        Ok(PublishPayload {
            application_message,
        })
    }

    fn parse_application_message(
        bytes: &mut impl ByteOperations,
    ) -> Result<String, MQTTProtocolError> {
        let application_message = utf::utf_8_handler::read(bytes)?;
        Ok(application_message)
    }
}

#[cfg(test)]
mod publish_payload_tests {
    use crate::protocol::mqtt4::payload_parser::publish::PublishPayload;
    use crate::utils::utf::utf_8_handler::write;
    use bytes::BytesMut;

    #[test]
    fn publish_payload_parser_should_parse_payload_correctly() {
        let mut bytes = BytesMut::new();
        let _ = write(&mut bytes, "Hello MQTT");

        let publish_payload = PublishPayload::parse(&mut bytes).unwrap();

        assert_eq!(publish_payload.application_message(), "Hello MQTT");
    }

    #[test]
    fn publish_payload_can_handle_empty_message() {
        let mut bytes = BytesMut::new();
        let _ = write(&mut bytes, "");

        let publish_payload = PublishPayload::parse(&mut bytes).unwrap();

        assert_eq!(publish_payload.application_message(), "");
    }
}
