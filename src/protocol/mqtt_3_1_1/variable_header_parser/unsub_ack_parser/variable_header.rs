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
#[derive(PartialEq, Debug)]
pub(crate) struct UnSubAckVariableHeader {
    packet_identifier: u16,
}

#[allow(dead_code)]
impl UnSubAckVariableHeader {
    pub(crate) fn new(packet_identifier: u16) -> Self {
        UnSubAckVariableHeader { packet_identifier }
    }

    pub fn packet_identifier(&self) -> u16 {
        self.packet_identifier
    }
}

#[cfg(test)]
mod unsubscribe_variable_header_tests {
    use crate::protocol::mqtt_3_1_1::variable_header_parser::unsub_ack_parser::variable_header::UnSubAckVariableHeader;
    use crate::protocol::mqtt_3_1_1::variable_header_parser::variable_header_codec::VariableHeaderEncoder;
    use bytes::BytesMut;

    #[test]
    fn unsub_ack_variable_parser_should_parse_variable_header_correctly() {
        let mut bytes = BytesMut::new();

        let expect_un_sub_ack_variable_header = UnSubAckVariableHeader::new(0x2211);
        let encode_expect_un_sub_ack_variable_header =
            expect_un_sub_ack_variable_header.encode().unwrap();
        bytes.extend(&encode_expect_un_sub_ack_variable_header);

        let unsubscribe_ack_variable_header = UnSubAckVariableHeader::decode(&mut bytes).unwrap();

        assert_eq!(
            unsubscribe_ack_variable_header.packet_identifier(),
            expect_un_sub_ack_variable_header.packet_identifier()
        );
    }
}
