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
use crate::protocol::mqtt4::payload_parser::mqtt_payload_codec::MqttPayloadDecoder;
use crate::protocol::mqtt4::payload_parser::unsubscribe_parser::payload::UnSubscribePayload;
use crate::protocol::mqtt4::variable_header_parser::unsubscribe::UnSubScribeVariableHeader;
use crate::utils::utf;

impl MqttPayloadDecoder<UnSubScribeVariableHeader> for UnSubscribePayload {
    fn decode(
        _fixed_header: &FixedHeader,
        _variable_header: &UnSubScribeVariableHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<Self, MqttProtocolError>
    where
        Self: Sized,
    {
        Self::decode(bytes)
    }
}

#[allow(dead_code)]
impl UnSubscribePayload {
    pub(super) fn decode(
        bytes: &mut impl ByteOperations,
    ) -> Result<UnSubscribePayload, MqttProtocolError> {
        let mut topics = Vec::new();

        while let Some(topic) = Self::parse_topic(bytes)? {
            topics.push(topic);
        }

        Self::verify_topics_is_empty(&mut topics)?;

        Ok(UnSubscribePayload::new(topics))
    }

    fn parse_topic(bytes: &mut impl ByteOperations) -> Result<Option<String>, MqttProtocolError> {
        if bytes.is_empty() {
            return Ok(None);
        }

        let topic_filter = Self::parse_topic_filter(bytes)?;

        Ok(Some(topic_filter))
    }

    fn parse_topic_filter(bytes: &mut impl ByteOperations) -> Result<String, MqttProtocolError> {
        let topic_filter = utf::utf_8_handler::read(bytes)?;
        Ok(topic_filter)
    }

    fn verify_topics_is_empty(topics: &mut [String]) -> Result<(), MqttProtocolError> {
        if topics.is_empty() {
            return Err(MqttProtocolError::MalformedPacket);
        }
        Ok(())
    }
}
