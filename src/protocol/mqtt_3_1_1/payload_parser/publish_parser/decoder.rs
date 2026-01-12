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
use crate::protocol::mqtt_3_1_1::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt_3_1_1::payload_parser::payload_codec::PayloadDecoder;
use crate::protocol::mqtt_3_1_1::payload_parser::publish_parser::payload::PublishPayload;
use crate::protocol::mqtt_3_1_1::variable_header_parser::publish_parser::variable_header::PublishVariableHeader;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::utils::utf;

impl PayloadDecoder<PublishVariableHeader> for PublishPayload {
    fn decode(
        _fixed_header: &FixedHeader,
        _variable_header: &PublishVariableHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<PublishPayload, MqttProtocolError> {
        Self::decode(bytes)
    }
}

#[allow(dead_code)]
impl PublishPayload {
    pub(super) fn decode(
        bytes: &mut impl ByteOperations,
    ) -> Result<PublishPayload, MqttProtocolError> {
        let application_message = Self::parse_application_message(bytes)?;
        Ok(PublishPayload::new(application_message))
    }

    fn parse_application_message(
        bytes: &mut impl ByteOperations,
    ) -> Result<String, MqttProtocolError> {
        let application_message = utf::utf_8_handler::read(bytes)?;
        Ok(application_message)
    }
}
