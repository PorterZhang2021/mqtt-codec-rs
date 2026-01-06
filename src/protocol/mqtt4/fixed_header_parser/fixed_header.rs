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

use crate::protocol::common::control_packet_type::ControlPacketType;
use crate::protocol::mqtt4::fixed_header_parser::fixed_header_flags::FixedHeaderFlags;
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedHeader {
    control_packet_type: ControlPacketType,
    fixed_header_reserved_flags: FixedHeaderFlags,
    remaining_length: u32,
}

#[allow(dead_code)]
impl FixedHeader {
    pub fn new(
        control_packet_type: ControlPacketType,
        fixed_header_reserved_flags: FixedHeaderFlags,
    ) -> FixedHeader {
        FixedHeader {
            control_packet_type,
            fixed_header_reserved_flags,
            remaining_length: 0,
        }
    }
    pub(super) fn self_create(
        control_packet_type: ControlPacketType,
        fixed_header_reserved_flags: FixedHeaderFlags,
        remaining_length: u32,
    ) -> FixedHeader {
        FixedHeader {
            control_packet_type,
            fixed_header_reserved_flags,
            remaining_length,
        }
    }

    pub(crate) fn control_packet_type(&self) -> &ControlPacketType {
        &self.control_packet_type
    }

    pub(crate) fn fixed_header_reserved_flags(&self) -> &FixedHeaderFlags {
        &self.fixed_header_reserved_flags
    }

    pub(crate) fn remaining_length(&self) -> u32 {
        self.remaining_length
    }

    pub(crate) fn set_remaining_length(&mut self, remaining_length: u32) {
        self.remaining_length = remaining_length;
    }
}

#[cfg(test)]
mod fixed_header_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::common::control_packet_type::ControlPacketType;
    use crate::protocol::common::qos::QoSCode;
    use crate::protocol::mqtt_protocol_error::MqttProtocolError;
    use crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader;
    use crate::protocol::mqtt4::fixed_header_parser::fixed_header_codec::MqttFixedHeaderEncoder;
    use crate::protocol::mqtt4::fixed_header_parser::fixed_header_flags::FixedHeaderFlags;
    use bytes::BytesMut;

    #[test]
    fn fixed_header_can_parse_connect_packet() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0b0001_0000, 0b0000_0010, 0b0000_0100]);
        let fixed_header = FixedHeader::decode(&mut bytes_mut).unwrap();
        assert_eq!(fixed_header.control_packet_type, ControlPacketType::Connect);
        assert_eq!(
            fixed_header.fixed_header_reserved_flags,
            FixedHeaderFlags::Connect
        );
        assert_eq!(fixed_header.remaining_length, 2);
        assert_eq!(bytes_mut.read_a_byte().unwrap(), 0b0000_0100);
    }

    #[test]
    fn fixed_header_can_encode_connect_packet() {
        let mut bytes_mut = BytesMut::new();
        let mut expect_fixed_header =
            FixedHeader::new(ControlPacketType::Connect, FixedHeaderFlags::Connect);
        let encode_expect_fixed_header = expect_fixed_header.encode(2).unwrap();
        bytes_mut.write_bytes(&encode_expect_fixed_header);

        let fixed_header = FixedHeader::decode(&mut bytes_mut).unwrap();
        assert_eq!(
            fixed_header.control_packet_type(),
            expect_fixed_header.control_packet_type()
        );
        assert_eq!(
            fixed_header.fixed_header_reserved_flags(),
            expect_fixed_header.fixed_header_reserved_flags()
        );
    }

    #[test]
    fn fixed_header_can_parse_publish_packet() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0b0011_1101, 0b0000_0011, 0b0000_0101, 0b0000_0110]);
        let fixed_header = FixedHeader::decode(&mut bytes_mut).unwrap();
        assert_eq!(fixed_header.control_packet_type, ControlPacketType::Publish);
        assert_eq!(
            fixed_header.fixed_header_reserved_flags,
            FixedHeaderFlags::Publish {
                dup: true,
                qos: QoSCode::Qos2,
                retain: true
            }
        );
        assert_eq!(fixed_header.remaining_length, 3);
        assert_eq!(bytes_mut.read_a_byte().unwrap(), 0b0000_0101);
    }

    #[test]
    fn fixed_header_can_encode_publish_packet() {
        let mut bytes_mut = BytesMut::new();
        let mut expect_fixed_header = FixedHeader::new(
            ControlPacketType::Publish,
            FixedHeaderFlags::Publish {
                dup: true,
                qos: QoSCode::Qos2,
                retain: true,
            },
        );
        let encode_expect_fixed_header = expect_fixed_header.encode(3).unwrap();
        bytes_mut.write_bytes(&encode_expect_fixed_header);

        let fixed_header = FixedHeader::decode(&mut bytes_mut).unwrap();
        assert_eq!(
            fixed_header.control_packet_type(),
            expect_fixed_header.control_packet_type()
        );
        assert_eq!(
            fixed_header.fixed_header_reserved_flags(),
            expect_fixed_header.fixed_header_reserved_flags()
        );
    }

    #[test]
    fn fixe_header_parse_fails_on_short_packet() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0b0001_0000]);
        let result = FixedHeader::decode(&mut bytes_mut);
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            MqttProtocolError::PacketTooShort
        ));
    }
}
