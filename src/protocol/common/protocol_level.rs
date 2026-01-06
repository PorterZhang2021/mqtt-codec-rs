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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtocolLevel {
    Mqtt3 = 3,
    Mqtt3_1_1 = 4,
    Mqtt5 = 5,
}

#[allow(dead_code)]
impl ProtocolLevel {
    pub(in crate::protocol) fn parse(level: u8) -> Result<ProtocolLevel, MqttProtocolError> {
        match level {
            3 => Ok(ProtocolLevel::Mqtt3),
            4 => Ok(ProtocolLevel::Mqtt3_1_1),
            5 => Ok(ProtocolLevel::Mqtt5),
            _ => Err(MqttProtocolError::ProtocolLevelNoSupport(level)),
        }
    }

    pub(in crate::protocol) fn as_u8(&self) -> u8 {
        match self {
            ProtocolLevel::Mqtt3 => 3,
            ProtocolLevel::Mqtt3_1_1 => 4,
            ProtocolLevel::Mqtt5 => 5,
        }
    }
}

#[cfg(test)]
mod protocol_level_tests {
    use crate::protocol::common::protocol_level::ProtocolLevel;
    use crate::protocol::mqtt_protocol_error::MqttProtocolError;

    #[test]
    fn protocol_level_parse_valid_levels() {
        let levels = vec![3, 4, 5];
        for level in levels {
            let result = ProtocolLevel::parse(level);
            assert!(result.is_ok());
            let protocol_level = result.unwrap();
            match level {
                0b0000_0011 => assert_eq!(protocol_level, ProtocolLevel::Mqtt3),
                0b0000_0100 => assert_eq!(protocol_level, ProtocolLevel::Mqtt3_1_1),
                0b0000_0101 => assert_eq!(protocol_level, ProtocolLevel::Mqtt5),
                _ => panic!("Unexpected level"),
            }
        }
    }

    #[test]
    fn protocol_level_parse_invalid_level_should_return_error() {
        let invalid_level = 6;
        let result = ProtocolLevel::parse(invalid_level);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(MqttProtocolError::ProtocolLevelNoSupport(level)) if level == invalid_level
        ));
    }
}
