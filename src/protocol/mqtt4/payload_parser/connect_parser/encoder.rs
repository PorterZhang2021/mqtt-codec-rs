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
use crate::protocol::mqtt4::payload_parser::connect_parser::payload::ConnectPayload;
use crate::protocol::mqtt4::payload_parser::mqtt_payload_codec::MqttPayloadEncoder;

impl MqttPayloadEncoder for ConnectPayload {
    fn encode(&self) -> Result<Vec<u8>, MqttProtocolError>
    where
        Self: Sized,
    {
        let mut encoded_bytes: Vec<u8> = Vec::new();

        let client_id_len = self.client_id().len() as u16;
        let encode_client_id_len = client_id_len.to_be_bytes().to_vec();

        let encode_client_id = self.client_id().as_bytes().to_vec();
        encoded_bytes.extend(encode_client_id_len);
        encoded_bytes.extend(encode_client_id);

        if let Some(will_topic) = self.will_topic() {
            let will_topic_len = will_topic.len() as u16;
            let encode_will_topic_len = will_topic_len.to_be_bytes().to_vec();
            let encode_will_topic = will_topic.as_bytes().to_vec();
            encoded_bytes.extend(encode_will_topic_len);
            encoded_bytes.extend(encode_will_topic);
        }

        if let Some(will_message) = self.will_message() {
            let will_message_len = will_message.len() as u16;
            let encode_will_message_len = will_message_len.to_be_bytes().to_vec();
            let encode_will_message = will_message.as_bytes().to_vec();
            encoded_bytes.extend(encode_will_message_len);
            encoded_bytes.extend(encode_will_message);
        }

        if let Some(username) = self.username() {
            let username_len = username.len() as u16;
            let encode_username_len = username_len.to_be_bytes().to_vec();
            let encode_username = username.as_bytes().to_vec();
            encoded_bytes.extend(encode_username_len);
            encoded_bytes.extend(encode_username);
        }

        if let Some(password) = self.password() {
            let password_len = password.len() as u16;
            let encode_password_len = password_len.to_be_bytes().to_vec();
            let encode_password = password.as_bytes().to_vec();
            encoded_bytes.extend(encode_password_len);
            encoded_bytes.extend(encode_password);
        }

        Ok(encoded_bytes)
    }
}
