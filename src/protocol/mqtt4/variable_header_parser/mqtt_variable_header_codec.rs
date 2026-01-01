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

#[allow(dead_code)]
pub(crate) trait MqttVariableHeaderDecoder {
    fn decode(
        fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<Self, MqttProtocolError>
    where
        Self: Sized;
}

#[allow(dead_code)]
pub(crate) trait MqttVariableHeaderEncoder<PayloadBytes> {
    fn encode(&self, payload_bytes: PayloadBytes) -> Result<Vec<u8>, MqttProtocolError>
    where
        Self: Sized;
}
