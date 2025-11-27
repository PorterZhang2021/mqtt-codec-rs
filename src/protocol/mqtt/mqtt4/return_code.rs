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

use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;

pub(crate) enum ReturnCode {
    ConnectionAccepted = 0,
    UnacceptableProtocolVersion = 1,
    IdentifierRejected = 2,
    ServerUnavailable = 3,
    BadUserNameOrPassword = 4,
    NotAuthorized = 5,
}

impl ReturnCode {
    pub(crate) fn parse(code: u8) -> Result<ReturnCode, MQTTProtocolError> {
        match code {
            0 => Ok(ReturnCode::ConnectionAccepted),
            1 => Ok(ReturnCode::UnacceptableProtocolVersion),
            2 => Ok(ReturnCode::IdentifierRejected),
            3 => Ok(ReturnCode::ServerUnavailable),
            4 => Ok(ReturnCode::BadUserNameOrPassword),
            5 => Ok(ReturnCode::NotAuthorized),
            _ => Err(MQTTProtocolError::ReservedReturnCode),
        }
    }
}

#[cfg(test)]
mod return_code_tests {
    use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
    use crate::protocol::mqtt::mqtt4::return_code::ReturnCode;

    #[test]
    fn parse_return_code_when_set_between_0_and_5() {
        for code in 0u8..=5u8 {
            let return_code = ReturnCode::parse(code).unwrap();
            match code {
                0 => assert!(matches!(return_code, ReturnCode::ConnectionAccepted)),
                1 => assert!(matches!(
                    return_code,
                    ReturnCode::UnacceptableProtocolVersion
                )),
                2 => assert!(matches!(return_code, ReturnCode::IdentifierRejected)),
                3 => assert!(matches!(return_code, ReturnCode::ServerUnavailable)),
                4 => assert!(matches!(return_code, ReturnCode::BadUserNameOrPassword)),
                5 => assert!(matches!(return_code, ReturnCode::NotAuthorized)),
                _ => panic!("Unexpected code"),
            }
        }
    }
    #[test]
    fn parse_return_code_when_set_out_of_range() {
        let invalid_codes = [6u8, 10u8, 255u8];
        for &code in &invalid_codes {
            let result = ReturnCode::parse(code);
            assert!(result.is_err());
            assert!(matches!(result, Err(MQTTProtocolError::ReservedReturnCode)));
        }
    }
}
