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
use crate::protocol::mqtt4::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderCodec;
use crate::utils::mqtt_utils;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub(crate) struct PubRecVariableHeader {
    packet_identifier: u16,
}

#[allow(dead_code)]
impl PubRecVariableHeader {
    pub fn new(packet_identifier: u16) -> Self {
        PubRecVariableHeader { packet_identifier }
    }

    pub fn packet_identifier(&self) -> u16 {
        self.packet_identifier
    }
}

#[allow(dead_code)]
impl MqttVariableHeaderCodec for PubRecVariableHeader {
    fn decode(
        _fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<PubRecVariableHeader, MqttProtocolError> {
        Self::parse(bytes)
    }

    fn encode(_variable_header: PubRecVariableHeader) -> Result<&'static [u8], MqttProtocolError> {
        todo!()
    }
}

#[allow(dead_code)]
impl PubRecVariableHeader {
    fn parse(bytes: &mut impl ByteOperations) -> Result<PubRecVariableHeader, MqttProtocolError> {
        let packet_identifier = mqtt_utils::parse_packet_identifier(bytes)?;
        Ok(PubRecVariableHeader { packet_identifier })
    }
}

#[cfg(test)]
mod pub_rec_variable_header_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::mqtt4::variable_header_parser::pub_rec::PubRecVariableHeader;
    use bytes::BytesMut;

    #[test]
    fn pub_rec_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0x12);
        bytes.write_a_byte(0x34);

        let pub_rec_variable_header = PubRecVariableHeader::parse(&mut bytes).unwrap();

        assert_eq!(pub_rec_variable_header.packet_identifier, 0x1234);
    }
}
