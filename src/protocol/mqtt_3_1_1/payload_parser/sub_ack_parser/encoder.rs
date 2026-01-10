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

use crate::protocol::mqtt_3_1_1::payload_parser::mqtt_payload_codec::MqttPayloadEncoder;
use crate::protocol::mqtt_3_1_1::payload_parser::sub_ack_parser::payload::SubAckPayload;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;

impl MqttPayloadEncoder for SubAckPayload {
    fn encode(&self) -> Result<Vec<u8>, MqttProtocolError>
    where
        Self: Sized,
    {
        let mut encoded_bytes: Vec<u8> = Vec::new();
        for return_code in self.return_codes() {
            encoded_bytes.push(return_code.as_u8());
        }
        Ok(encoded_bytes)
    }
}
