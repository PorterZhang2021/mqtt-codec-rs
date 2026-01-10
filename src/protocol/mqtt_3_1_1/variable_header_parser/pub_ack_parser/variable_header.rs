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

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct PubAckVariableHeader {
    packet_identifier: u16,
}

#[allow(dead_code)]
impl PubAckVariableHeader {
    pub fn new(packet_identifier: u16) -> Self {
        PubAckVariableHeader { packet_identifier }
    }

    pub fn packet_identifier(&self) -> u16 {
        self.packet_identifier
    }
}

#[cfg(test)]
mod pub_ack_variable_header_tests {
    use crate::protocol::mqtt_3_1_1::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderEncoder;
    use crate::protocol::mqtt_3_1_1::variable_header_parser::pub_ack_parser::variable_header::PubAckVariableHeader;
    use bytes::BytesMut;

    #[test]
    fn pub_ack_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();
        let expect_pub_ack_variable_header = PubAckVariableHeader::new(10);
        let encode_pub_ack_variable_header = expect_pub_ack_variable_header.encode().unwrap();
        bytes.extend(encode_pub_ack_variable_header);

        let pub_ack_variable_header = PubAckVariableHeader::decode(&mut bytes).unwrap();

        assert_eq!(expect_pub_ack_variable_header, pub_ack_variable_header);
    }
}
