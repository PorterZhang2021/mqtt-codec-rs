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
use crate::protocol::mqtt4::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderEncoder;
use crate::protocol::mqtt4::variable_header_parser::publish_parser::variable_header::PublishVariableHeader;
use crate::utils::radix;

impl MqttVariableHeaderEncoder for PublishVariableHeader {
    fn encode(&self, payload_bytes: Vec<u8>) -> Result<Vec<u8>, MqttProtocolError>
    where
        Self: Sized,
    {
        let mut bytes: Vec<u8> = Vec::new();
        let encode_topic_name_len =
            radix::radix_handler::u16_to_be_2_bytes(self.topic_name().len())?.to_vec();
        let encode_topic_name = self.topic_name().as_bytes().to_vec();
        bytes.extend(encode_topic_name_len);
        bytes.extend(encode_topic_name);
        if let Some(packet_identifier) = self.packet_identifier() {
            let encode_packet_identifier = packet_identifier.to_be_bytes().to_vec();
            bytes.extend(encode_packet_identifier);
        }
        bytes.extend(payload_bytes);
        Ok(bytes)
    }
}
