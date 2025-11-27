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
use crate::protocol::mqtt_protocol_error::MQTTProtocolError;
use crate::utils::mqtt_utils;

#[allow(dead_code)]
pub(crate) struct PubAckVariableHeader {
    packet_identifier: u16,
}

#[allow(dead_code)]
impl PubAckVariableHeader {
    fn parse(bytes: &mut impl ByteOperations) -> Result<PubAckVariableHeader, MQTTProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(PubAckVariableHeader { packet_identifier })
    }
}

#[cfg(test)]
mod pub_ack_variable_header_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::mqtt4::variable_header_parser::pub_ack::PubAckVariableHeader;
    use bytes::BytesMut;

    #[test]
    fn pub_ack_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0x00);
        bytes.write_a_byte(0x0A);

        let pub_ack_variable_header = PubAckVariableHeader::parse(&mut bytes).unwrap();

        assert_eq!(pub_ack_variable_header.packet_identifier, 10);
    }
}
