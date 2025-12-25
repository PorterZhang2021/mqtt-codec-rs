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

use crate::byte_adapter::byte_operations::ByteOperations;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::protocol::mqtt4::payload_parser::connect_parser::payload::ConnectPayload;
use crate::protocol::mqtt4::payload_parser::mqtt_payload_codec::MqttPayloadDecoder;
use crate::protocol::mqtt4::variable_header_parser::connect::ConnectVariableHeader;
use crate::utils::utf;

impl MqttPayloadDecoder<ConnectVariableHeader> for ConnectPayload {
    fn decode(
        _fixed_header: &crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader,
        variable_header: &ConnectVariableHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnectPayload, MqttProtocolError>
    where
        Self: Sized,
    {
        Self::decode(bytes, variable_header)
    }
}

#[allow(dead_code)]
impl ConnectPayload {
    pub(super) fn decode(
        bytes: &mut impl ByteOperations,
        connect_variable_header: &ConnectVariableHeader,
    ) -> Result<ConnectPayload, MqttProtocolError> {
        let client_id = Self::parse_client_id(bytes)?;

        if client_id.is_empty() && !connect_variable_header.connect_flags().clean_session() {
            return Err(MqttProtocolError::InvalidClientId);
        }

        let mut will_topic: Option<String> = None;
        let mut will_message: Option<String> = None;
        if connect_variable_header.connect_flags().will_flag() {
            will_topic = Some(Self::parse_will_topic(bytes)?);
            will_message = Some(Self::parse_will_message(bytes)?);
        }

        let mut username: Option<String> = None;
        let mut password: Option<String> = None;
        if connect_variable_header.connect_flags().username_flag() {
            username = Some(Self::parse_username(bytes)?);
        }
        if connect_variable_header.connect_flags().password_flag() {
            password = Some(Self::parse_password(bytes)?);
        }

        Ok(ConnectPayload::new(
            client_id,
            will_topic,
            will_message,
            username,
            password,
        ))
    }

    fn parse_password(bytes: &mut impl ByteOperations) -> Result<String, MqttProtocolError> {
        let password = utf::utf_8_handler::read(bytes)?;
        Ok(password)
    }

    fn parse_username(bytes: &mut impl ByteOperations) -> Result<String, MqttProtocolError> {
        let username = utf::utf_8_handler::read(bytes)?;
        Self::verify_user_name(&username)?;
        Ok(username)
    }

    fn verify_user_name(username: &str) -> Result<(), MqttProtocolError> {
        if username.is_empty() {
            return Err(MqttProtocolError::MalformedPacket);
        }
        Ok(())
    }

    fn parse_will_message(bytes: &mut impl ByteOperations) -> Result<String, MqttProtocolError> {
        let will_topic = utf::utf_8_handler::read(bytes)?;
        Ok(will_topic)
    }

    fn parse_will_topic(bytes: &mut impl ByteOperations) -> Result<String, MqttProtocolError> {
        let will_message = utf::utf_8_handler::read(bytes)?;
        Ok(will_message)
    }

    pub(super) fn parse_client_id(
        bytes: &mut impl ByteOperations,
    ) -> Result<String, MqttProtocolError> {
        let client_id = utf::utf_8_handler::read(bytes)?;
        Self::verify_string_is_ascii_alphanumeric(&client_id)?;
        Self::verify_client_id_length(&client_id)?;
        Ok(client_id)
    }

    fn verify_string_is_ascii_alphanumeric(client_id: &str) -> Result<(), MqttProtocolError> {
        if !client_id.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(MqttProtocolError::InvalidClientId);
        }
        Ok(())
    }

    fn verify_client_id_length(client_id: &str) -> Result<(), MqttProtocolError> {
        let length = client_id.len();
        if length > 23 {
            return Err(MqttProtocolError::InvalidClientId);
        }
        Ok(())
    }
}
