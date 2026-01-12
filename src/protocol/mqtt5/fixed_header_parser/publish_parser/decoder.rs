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
use crate::protocol::mqtt5::fixed_header_parser::fixed_header_codec::FixedHeaderDecoder;
use crate::protocol::mqtt5::fixed_header_parser::publish_parser::fixed_header::{
    PublishFixedHeader, PublishFixedHeaderReservedFlags,
};

#[allow(dead_code)]
impl FixedHeaderDecoder for PublishFixedHeader {
    fn decode(bytes: &mut impl ByteOperations) -> Result<Self, MqttProtocolError>
    where
        Self: Sized,
    {
        let first_byte = bytes
            .read_a_byte()
            .ok_or(MqttProtocolError::PacketTooShort)?;
        let packet_type = ControlPacketType::parse(first_byte)?;

        let FixedHeaderFlags::Publish { dup, qos, retain } =
            FixedHeaderFlags::parse(&packet_type, first_byte)?
        else {
            return Err(MqttProtocolError::MalformedPacket);
        };

        let fixed_header_reserved_flags = PublishFixedHeaderReservedFlags::new(dup, qos, retain);
        let remaining_len = remaining_length_parser::parse(bytes)?;

        Ok(PublishFixedHeader::self_create(
            packet_type,
            fixed_header_reserved_flags,
            remaining_len,
        ))
    }
}

#[cfg(test)]
mod publish_fixed_header_decoder_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::common::control_packet_type::ControlPacketType;
    use crate::protocol::common::qos::QoSCode::Qos1;
    use crate::protocol::common::remaining_length::remaining_length_parser;
    use crate::protocol::mqtt5::fixed_header_parser::fixed_header_codec::FixedHeaderDecoder;
    use crate::protocol::mqtt5::fixed_header_parser::publish_parser::fixed_header::{
        PublishFixedHeader, PublishFixedHeaderReservedFlags,
    };
    use bytes::BytesMut;

    #[test]
    fn publish_fixed_header_should_control_packet_type_publish() {
        let mut bytes_mut = BytesMut::new();
        let first_byte = 0b0011_0000;
        bytes_mut.write_a_byte(first_byte);
        let remaining_length_byte = remaining_length_parser::encode(0).unwrap();
        bytes_mut.write_bytes(remaining_length_byte.as_ref());
        let fixed_header = PublishFixedHeader::decode(&mut bytes_mut).unwrap();
        assert_eq!(
            fixed_header.control_packet_type(),
            &ControlPacketType::Publish
        )
    }

    #[test]
    fn publish_fixed_header_has_fixed_header_reserved_flags() {
        let mut bytes_mut = BytesMut::new();
        let first_byte = 0b0011_1011;
        bytes_mut.write_a_byte(first_byte);
        let remaining_length_byte = remaining_length_parser::encode(0).unwrap();
        bytes_mut.write_bytes(remaining_length_byte.as_ref());
        let fixed_header = PublishFixedHeader::decode(&mut bytes_mut).unwrap();
        let expected_flags = PublishFixedHeaderReservedFlags::new(true, Qos1, true);
        assert_eq!(fixed_header.fixed_header_reserved_flags(), &expected_flags)
    }

    #[test]
    fn publish_fixed_header_should_return_error_when_fixed_header_reserved_is_error() {
        let mut bytes_mut = BytesMut::new();
        let first_byte = 0b0011_1100;
        bytes_mut.write_a_byte(first_byte);
        let result = PublishFixedHeader::decode(&mut bytes_mut);
        assert!(result.is_err());
    }

    #[test]
    fn publish_fixed_header_should_get_remaining_length() {
        let mut bytes_mut = BytesMut::new();
        let first_byte = 0b0011_0000;
        bytes_mut.write_a_byte(first_byte);
        let expect_remaining_len = 200;
        let remaining_length_byte = remaining_length_parser::encode(expect_remaining_len).unwrap();
        bytes_mut.write_bytes(remaining_length_byte.as_ref());
        let fixed_header = PublishFixedHeader::decode(&mut bytes_mut).unwrap();
        assert_eq!(fixed_header.remaining_length(), expect_remaining_len);
    }
}
