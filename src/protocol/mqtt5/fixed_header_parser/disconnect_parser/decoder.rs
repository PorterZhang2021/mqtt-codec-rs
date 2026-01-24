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
use crate::protocol::mqtt5::fixed_header_parser::disconnect_parser::fixed_header::DisconnectFixedHeader;
use crate::protocol::mqtt5::fixed_header_parser::fixed_header_codec::{
    FixedHeaderDecoder, parse_fixed_header_by_default,
};

impl FixedHeaderDecoder for DisconnectFixedHeader {
    fn decode(bytes: &mut impl ByteOperations) -> Result<Self, MqttProtocolError>
    where
        Self: Sized,
    {
        parse_fixed_header_by_default(bytes, |control_packet_type, header| {
            DisconnectFixedHeader::self_create(control_packet_type, header)
        })
    }
}

#[cfg(test)]
mod disconnect_fixed_header_decoder_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::common::control_packet_type::ControlPacketType;
    use crate::protocol::common::remaining_length::remaining_length_parser;
    use crate::protocol::mqtt5::fixed_header_parser::disconnect_parser::fixed_header::DisconnectFixedHeader;
    use crate::protocol::mqtt5::fixed_header_parser::fixed_header_codec::FixedHeaderDecoder;
    use bytes::BytesMut;

    #[test]
    fn disconnect_fixed_header_should_control_packet_type_disconnect() {
        let mut bytes_mut = BytesMut::new();
        let first_byte = 0b1110_0000;
        bytes_mut.write_a_byte(first_byte);
        let remaining_length_byte = remaining_length_parser::encode(0).unwrap();
        bytes_mut.write_bytes(remaining_length_byte.as_ref());
        let fixed_header = DisconnectFixedHeader::decode(&mut bytes_mut).unwrap();
        assert_eq!(
            fixed_header.control_packet_type(),
            &ControlPacketType::Disconnect
        )
    }

    #[test]
    fn disconnect_fixed_header_should_return_error_when_fixed_header_reserved_is_error() {
        let mut bytes_mut = BytesMut::new();
        let first_byte = 0b1110_0100;
        bytes_mut.write_a_byte(first_byte);
        let result = DisconnectFixedHeader::decode(&mut bytes_mut);
        assert!(result.is_err());
    }

    #[test]
    fn disconnect_fixed_header_should_get_remaining_length() {
        let mut bytes_mut = BytesMut::new();
        let first_byte = 0b1110_0000;
        bytes_mut.write_a_byte(first_byte);
        let expect_remaining_len = 145;
        let remaining_length_byte = remaining_length_parser::encode(expect_remaining_len).unwrap();
        bytes_mut.write_bytes(remaining_length_byte.as_ref());
        let fixed_header = DisconnectFixedHeader::decode(&mut bytes_mut).unwrap();
        assert_eq!(fixed_header.remaining_length(), expect_remaining_len);
    }
}
