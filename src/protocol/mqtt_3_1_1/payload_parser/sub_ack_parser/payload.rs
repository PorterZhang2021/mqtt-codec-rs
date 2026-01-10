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

use crate::protocol::mqtt_protocol_error::MqttProtocolError;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct SubAckPayload {
    return_codes: Vec<SubAckReturnCode>,
}

#[allow(dead_code)]
impl SubAckPayload {
    pub fn new(return_codes: Vec<SubAckReturnCode>) -> Self {
        SubAckPayload { return_codes }
    }

    pub fn return_codes(&self) -> &Vec<SubAckReturnCode> {
        &self.return_codes
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SubAckReturnCode {
    Qos0 = 0,
    Qos1 = 1,
    Qos2 = 2,
    Failure = 0b1000_0000,
}

#[allow(dead_code)]
impl SubAckReturnCode {
    pub(super) fn parse(byte: u8) -> Result<SubAckReturnCode, MqttProtocolError> {
        match byte {
            0 => Ok(SubAckReturnCode::Qos0),
            1 => Ok(SubAckReturnCode::Qos1),
            2 => Ok(SubAckReturnCode::Qos2),
            0b1000_0000 => Ok(SubAckReturnCode::Failure),
            _ => Err(MqttProtocolError::MalformedPacket),
        }
    }
    pub(super) fn as_u8(&self) -> u8 {
        match self {
            SubAckReturnCode::Qos0 => 0,
            SubAckReturnCode::Qos1 => 1,
            SubAckReturnCode::Qos2 => 2,
            SubAckReturnCode::Failure => 0b1000_0000,
        }
    }
}

#[cfg(test)]
mod sub_ack_payload_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::mqtt_3_1_1::payload_parser::payload_codec::PayloadEncoder;
    use crate::protocol::mqtt_3_1_1::payload_parser::sub_ack_parser::payload::SubAckPayload;
    use crate::protocol::mqtt_3_1_1::payload_parser::sub_ack_parser::payload::SubAckReturnCode;
    use crate::protocol::mqtt_protocol_error::MqttProtocolError;
    use bytes::BytesMut;

    #[test]
    fn sub_ack_payload_parser_should_parse_payload_correctly() {
        let sub_ack_vec = vec![
            SubAckReturnCode::Qos0,
            SubAckReturnCode::Qos1,
            SubAckReturnCode::Failure,
        ];
        let expect_sub_ack_payload = SubAckPayload::new(sub_ack_vec);
        let encode_sub_ack_payload = expect_sub_ack_payload.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encode_sub_ack_payload);
        let sub_ack_payload = SubAckPayload::decode(&mut bytes).unwrap();
        assert_eq!(
            sub_ack_payload.return_codes().len(),
            expect_sub_ack_payload.return_codes().len()
        );
        assert_eq!(
            sub_ack_payload.return_codes()[0],
            expect_sub_ack_payload.return_codes()[0]
        );
        assert_eq!(
            sub_ack_payload.return_codes()[1],
            expect_sub_ack_payload.return_codes()[1]
        );
        assert_eq!(
            sub_ack_payload.return_codes()[2],
            expect_sub_ack_payload.return_codes()[2]
        );
    }

    #[test]
    fn sub_ack_payload_failure_test() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(3); // Invalid return code
        let result = SubAckPayload::decode(&mut bytes);
        assert!(matches!(result, Err(MqttProtocolError::MalformedPacket)));
    }
}
