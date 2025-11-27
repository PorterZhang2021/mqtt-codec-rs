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

use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::radix::radix_handler;

#[allow(dead_code)]
#[inline]
pub(crate) fn parse_packet_identifier(
    bytes: &mut impl ByteOperations,
) -> Result<u16, MQTTProtocolError> {
    let byte = bytes.read_bytes(2);
    Ok(radix_handler::be_bytes_to_u16(byte.as_slice())?)
}

#[cfg(test)]
mod mqtt_utils_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::utils::mqtt_utils;
    use bytes::BytesMut;

    #[test]
    fn mqtt_utils_should_parse_packet_identifier_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0x12);
        bytes.write_a_byte(0x34);

        let packet_identifier = mqtt_utils::parse_packet_identifier(&mut bytes).unwrap();

        assert_eq!(packet_identifier, 0x1234);
    }
}
