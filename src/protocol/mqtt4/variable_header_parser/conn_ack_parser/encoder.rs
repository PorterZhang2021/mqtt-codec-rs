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
use crate::protocol::mqtt4::variable_header_parser::conn_ack_parser::variable_header::ConnAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderEncoder;

impl MqttVariableHeaderEncoder for ConnAckVariableHeader {
    fn encode(&self, payload_bytes: Vec<u8>) -> Result<Vec<u8>, MqttProtocolError>
    where
        Self: Sized,
    {
        let mut bytes: Vec<u8> = Vec::new();
        // Session Present
        let session_present_byte: u8 = if self.session_present() {
            0b0000_0001
        } else {
            0b0000_0000
        };
        bytes.push(session_present_byte);

        // Return Code
        bytes.push(self.return_code().as_u8());

        // Append Payload
        bytes.extend(payload_bytes);

        Ok(bytes)
    }
}
