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

use crate::byte_wrapper::byte_operations::ByteOperations;
use crate::mqtt::mqtt_protocol_error::MQTTProtocolError;

#[allow(dead_code)]
struct SubAckPayload {
    return_codes: Vec<SubAckReturnCode>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum SubAckReturnCode {
    Qos0 = 0,
    Qos1 = 1,
    Qos2 = 2,
    Failure = 0b1000_0000,
}

#[allow(dead_code)]
impl SubAckReturnCode {
    fn parse(byte: u8) -> Result<SubAckReturnCode, MQTTProtocolError> {
        match byte {
            0 => Ok(SubAckReturnCode::Qos0),
            1 => Ok(SubAckReturnCode::Qos1),
            2 => Ok(SubAckReturnCode::Qos2),
            0b1000_0000 => Ok(SubAckReturnCode::Failure),
            _ => Err(MQTTProtocolError::MalformedPacket),
        }
    }
    fn as_u8(&self) -> u8 {
        match self {
            SubAckReturnCode::Qos0 => 0,
            SubAckReturnCode::Qos1 => 1,
            SubAckReturnCode::Qos2 => 2,
            SubAckReturnCode::Failure => 0b1000_0000,
        }
    }
}

#[allow(dead_code)]
impl SubAckPayload {
    fn parse(bytes: &mut impl ByteOperations) -> Result<SubAckPayload, MQTTProtocolError> {
        let mut return_codes = Vec::new();
        while let Some(code_byte) = bytes.read_a_byte() {
            let return_code = SubAckReturnCode::parse(code_byte)?;
            return_codes.push(return_code);
        }
        Ok(SubAckPayload { return_codes })
    }
}

#[cfg(test)]
mod sub_ack_payload_tests {
    use crate::byte_wrapper::byte_operations::ByteOperations;
    use crate::mqtt::mqtt_protocol_error::MQTTProtocolError;
    use crate::mqtt::mqtt4::payload_parser::sub_ack::{SubAckPayload, SubAckReturnCode};
    use bytes::BytesMut;

    #[test]
    fn sub_ack_payload_parser_should_parse_payload_correctly() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(0);
        bytes.write_a_byte(1);
        bytes.write_a_byte(0b1000_0000);
        let sub_ack_payload = SubAckPayload::parse(&mut bytes).unwrap();
        assert_eq!(sub_ack_payload.return_codes.len(), 3);
        assert_eq!(sub_ack_payload.return_codes[0], SubAckReturnCode::Qos0);
        assert_eq!(sub_ack_payload.return_codes[1], SubAckReturnCode::Qos1);
        assert_eq!(sub_ack_payload.return_codes[2], SubAckReturnCode::Failure);
    }

    #[test]
    fn sub_ack_payload_failure_test() {
        let mut bytes = BytesMut::new();
        bytes.write_a_byte(3); // Invalid return code
        let result = SubAckPayload::parse(&mut bytes);
        assert!(matches!(result, Err(MQTTProtocolError::MalformedPacket)));
    }
}
