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
use crate::protocol::mqtt_3_1_1::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt_3_1_1::fixed_header_parser::fixed_header_codec::FixedHeaderDecoder;
use crate::protocol::mqtt_3_1_1::fixed_header_parser::remaining_length::remaining_length_parser;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;

impl FixedHeaderDecoder for FixedHeader {
    fn decode(bytes: &mut impl ByteOperations) -> Result<Self, MqttProtocolError> {
        Self::decode(bytes)
    }
}

#[allow(dead_code)]
impl FixedHeader {
    pub(crate) fn decode(
        bytes: &mut impl ByteOperations,
    ) -> Result<FixedHeader, MqttProtocolError> {
        let first_byte = bytes
            .read_a_byte()
            .ok_or(MqttProtocolError::PacketTooShort)?;
        let control_packet_type = ControlPacketType::parse(first_byte)?;

        let fixed_header_reserve_flags = FixedHeaderFlags::parse(&control_packet_type, first_byte)?;

        let remaining_length = remaining_length_parser::parse(bytes)?;

        Ok(FixedHeader::self_create(
            control_packet_type,
            fixed_header_reserve_flags,
            remaining_length,
        ))
    }
}
