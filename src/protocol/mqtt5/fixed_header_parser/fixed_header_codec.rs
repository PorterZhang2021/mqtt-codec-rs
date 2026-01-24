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
use crate::protocol::common::control_packet_type::ControlPacketType;
use crate::protocol::common::fixed_header_flags::FixedHeaderFlags;
use crate::protocol::common::remaining_length::remaining_length_parser;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;

#[allow(dead_code)]
pub(crate) trait FixedHeaderDecoder {
    fn decode(bytes: &mut impl ByteOperations) -> Result<Self, MqttProtocolError>
    where
        Self: Sized;
}

pub fn parse_fixed_header_by_default<T, F>(
    bytes: &mut impl ByteOperations,
    constructor: F,
) -> Result<T, MqttProtocolError>
where
    F: FnOnce(ControlPacketType, u32) -> T,
{
    let first_byte = bytes
        .read_a_byte()
        .ok_or(MqttProtocolError::PacketTooShort)?;
    let packet_type = ControlPacketType::parse(first_byte)?;
    FixedHeaderFlags::parse(&packet_type, first_byte)?;
    let remaining_len = remaining_length_parser::parse(bytes)?;

    Ok(constructor(packet_type, remaining_len))
}
