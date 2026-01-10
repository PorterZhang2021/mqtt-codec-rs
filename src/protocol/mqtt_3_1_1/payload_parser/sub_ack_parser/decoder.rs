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
use crate::protocol::mqtt_3_1_1::payload_parser::mqtt_payload_codec::MqttPayloadDecoder;
use crate::protocol::mqtt_3_1_1::payload_parser::sub_ack_parser::payload::{
    SubAckPayload, SubAckReturnCode,
};
use crate::protocol::mqtt_3_1_1::variable_header_parser::sub_ack_parser::variable_header::SubAckVariableHeader;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;

impl MqttPayloadDecoder<SubAckVariableHeader> for SubAckPayload {
    fn decode(
        _fixed_header: &FixedHeader,
        _variable_header: &SubAckVariableHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<SubAckPayload, MqttProtocolError> {
        Self::decode(bytes)
    }
}

#[allow(dead_code)]
impl SubAckPayload {
    pub(super) fn decode(
        bytes: &mut impl ByteOperations,
    ) -> Result<SubAckPayload, MqttProtocolError> {
        let mut return_codes = Vec::new();
        while let Some(code_byte) = bytes.read_a_byte() {
            let return_code = SubAckReturnCode::parse(code_byte)?;
            return_codes.push(return_code);
        }
        Ok(SubAckPayload::new(return_codes))
    }
}
