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
use crate::protocol::mqtt_3_1_1::variable_header_parser::subscribe_parser::variable_header::SubscribeVariableHeader;
use crate::protocol::mqtt_3_1_1::variable_header_parser::variable_header_codec::VariableHeaderDecoder;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::utils::mqtt_utils;

#[allow(dead_code)]
impl VariableHeaderDecoder for SubscribeVariableHeader {
    fn decode(
        _fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<SubscribeVariableHeader, MqttProtocolError> {
        Self::decode(bytes)
    }
}

#[allow(dead_code)]
impl SubscribeVariableHeader {
    pub(super) fn decode(
        bytes: &mut impl ByteOperations,
    ) -> Result<SubscribeVariableHeader, MqttProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(SubscribeVariableHeader::new(packet_identifier))
    }
}
