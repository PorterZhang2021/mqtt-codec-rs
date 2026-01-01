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
#[derive(Debug, PartialEq)]
pub enum QoSCode {
    Qos0 = 0,
    Qos1 = 1,
    Qos2 = 2,
}

#[allow(dead_code)]
impl QoSCode {
    pub(in crate::protocol) fn parse(byte: u8) -> Result<QoSCode, MqttProtocolError> {
        match byte {
            0 => Ok(QoSCode::Qos0),
            1 => Ok(QoSCode::Qos1),
            2 => Ok(QoSCode::Qos2),
            _ => Err(MqttProtocolError::MalformedPacket),
        }
    }
    pub(in crate::protocol) fn as_u8(&self) -> u8 {
        match self {
            QoSCode::Qos0 => 0,
            QoSCode::Qos1 => 1,
            QoSCode::Qos2 => 2,
        }
    }
}

#[cfg(test)]
mod qos_code_tests {
    use crate::protocol::common::qos::QoSCode;
    use crate::protocol::mqtt_protocol_error::MqttProtocolError;
    #[test]
    fn qos_code_parse_valid_codes() {
        let codes = vec![0, 1, 2];
        for code in codes {
            let result = QoSCode::parse(code);
            assert!(result.is_ok());
            let qos_code = result.unwrap();
            match code {
                0b0000_0000 => assert_eq!(qos_code, QoSCode::Qos0),
                0b0000_0001 => assert_eq!(qos_code, QoSCode::Qos1),
                0b0000_0010 => assert_eq!(qos_code, QoSCode::Qos2),
                _ => panic!("Unexpected code"),
            }
        }
    }

    #[test]
    fn qos_code_parse_invalid_code_should_return_error() {
        let invalid_code = 3;
        let result = QoSCode::parse(invalid_code);
        assert!(result.is_err());
        assert!(matches!(result, Err(MqttProtocolError::MalformedPacket)));
    }
}
