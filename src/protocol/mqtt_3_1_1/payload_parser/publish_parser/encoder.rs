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
use crate::protocol::mqtt_3_1_1::payload_parser::publish_parser::payload::PublishPayload;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::utils::radix::radix_handler;

impl MqttPayloadEncoder for PublishPayload {
    fn encode(&self) -> Result<Vec<u8>, MqttProtocolError>
    where
        Self: Sized,
    {
        let mut encoded_bytes: Vec<u8> = Vec::new();

        let encode_application_message_len =
            radix_handler::u16_to_be_2_bytes(self.application_message().len())?.to_vec();

        let encode_application_message = self.application_message().as_bytes().to_vec();
        encoded_bytes.extend(encode_application_message_len);
        encoded_bytes.extend(encode_application_message);

        Ok(encoded_bytes)
    }
}
