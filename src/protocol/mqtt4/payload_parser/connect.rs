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
use crate::protocol::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::mqtt4::payload_parser::mqtt_payload_codec::MqttPayloadCodec;
use crate::protocol::mqtt4::variable_header_parser::connect::ConnectVariableHeader;
use crate::utils::utf;

#[allow(dead_code)]
pub(crate) struct ConnectPayload {
    client_id: String,
    will_topic: Option<String>,
    will_message: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[allow(dead_code)]
impl ConnectPayload {
    fn client_id(&self) -> &str {
        &self.client_id
    }

    fn will_topic(&self) -> Option<&str> {
        self.will_topic.as_deref()
    }

    fn will_message(&self) -> Option<&str> {
        self.will_message.as_deref()
    }

    fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }
}

impl MqttPayloadCodec<ConnectVariableHeader> for ConnectPayload {
    fn decode(
        _fixed_header: &crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader,
        variable_header: &ConnectVariableHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnectPayload, MQTTProtocolError>
    where
        Self: Sized,
    {
        Self::parse(bytes, variable_header)
    }

    fn encode(_payload: Self) -> Result<&'static [u8], MQTTProtocolError> {
        todo!()
    }
}

#[allow(dead_code)]
impl ConnectPayload {
    fn parse(
        bytes: &mut impl ByteOperations,
        connect_variable_header: &ConnectVariableHeader,
    ) -> Result<ConnectPayload, MQTTProtocolError> {
        let client_id = Self::parse_client_id(bytes)?;

        if client_id.is_empty() && !connect_variable_header.connect_flags().clean_session {
            return Err(MQTTProtocolError::InvalidClientId);
        }

        let mut will_topic: Option<String> = None;
        let mut will_message: Option<String> = None;
        if connect_variable_header.connect_flags().will_flag {
            will_topic = Some(Self::parse_will_topic(bytes)?);
            will_message = Some(Self::parse_will_message(bytes)?);
        }

        let mut username: Option<String> = None;
        let mut password: Option<String> = None;
        if connect_variable_header.connect_flags().username_flag {
            username = Some(Self::parse_username(bytes)?);
        }
        if connect_variable_header.connect_flags().password_flag {
            password = Some(Self::parse_password(bytes)?);
        }

        Ok(ConnectPayload {
            client_id,
            will_topic,
            will_message,
            username,
            password,
        })
    }

    fn parse_password(bytes: &mut impl ByteOperations) -> Result<String, MQTTProtocolError> {
        let password = utf::utf_8_handler::read(bytes)?;
        Ok(password)
    }

    fn parse_username(bytes: &mut impl ByteOperations) -> Result<String, MQTTProtocolError> {
        let username = utf::utf_8_handler::read(bytes)?;
        Self::verify_user_name(&username)?;
        Ok(username)
    }

    fn verify_user_name(username: &str) -> Result<(), MQTTProtocolError> {
        if username.is_empty() {
            return Err(MQTTProtocolError::MalformedPacket);
        }
        Ok(())
    }

    fn parse_will_message(bytes: &mut impl ByteOperations) -> Result<String, MQTTProtocolError> {
        let will_topic = utf::utf_8_handler::read(bytes)?;
        Ok(will_topic)
    }

    fn parse_will_topic(bytes: &mut impl ByteOperations) -> Result<String, MQTTProtocolError> {
        let will_message = utf::utf_8_handler::read(bytes)?;
        Ok(will_message)
    }

    fn parse_client_id(bytes: &mut impl ByteOperations) -> Result<String, MQTTProtocolError> {
        let client_id = utf::utf_8_handler::read(bytes)?;
        Self::verify_string_is_ascii_alphanumeric(&client_id)?;
        Self::verify_client_id_length(&client_id)?;
        Ok(client_id)
    }

    fn verify_string_is_ascii_alphanumeric(client_id: &str) -> Result<(), MQTTProtocolError> {
        if !client_id.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(MQTTProtocolError::InvalidClientId);
        }
        Ok(())
    }

    fn verify_client_id_length(client_id: &str) -> Result<(), MQTTProtocolError> {
        let length = client_id.len();
        if length > 23 {
            return Err(MQTTProtocolError::InvalidClientId);
        }
        Ok(())
    }
}

#[cfg(test)]
mod connect_payload_tests {
    use crate::protocol::mqtt4::payload_parser::connect::ConnectPayload;
    use crate::protocol::mqtt4::variable_header_parser::connect::{
        ConnectFlags, ConnectVariableHeader,
    };
    use crate::utils::utf;
    use bytes::BytesMut;

    #[test]
    fn client_id_should_contain_only_valid_characters() {
        let valid_client_id = "Client123";
        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, valid_client_id).unwrap();
        let result = ConnectPayload::parse_client_id(&mut bytes).unwrap();
        assert_eq!(result, valid_client_id);
    }

    #[test]
    fn client_id_with_invalid_characters_should_return_error() {
        let invalid_client_id = "Client@123";
        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, invalid_client_id).unwrap();
        let result = ConnectPayload::parse_client_id(&mut bytes);
        assert!(result.is_err());
    }

    #[test]
    fn client_id_can_be_zero_length_if_clean_session_is_true() {
        let clean_session = true;
        let connect_flags =
            ConnectFlags::new(false, false, false, 0, false, clean_session).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);
        let client_id = "";
        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, client_id).unwrap();
        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
    }

    #[test]
    fn client_id_cannot_be_zero_length_if_clean_session_is_false() {
        let clean_session = false;
        let connect_flags =
            ConnectFlags::new(false, false, false, 0, false, clean_session).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);
        let client_id = "";
        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, client_id).unwrap();
        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_err());
    }

    #[test]
    fn client_id_length_between_1_and_23_bytes() {
        for length in 1..=23 {
            let client_id: String = "A".repeat(length);
            let mut bytes = BytesMut::new();
            utf::utf_8_handler::write(&mut bytes, &client_id).unwrap();
            let result = ConnectPayload::parse_client_id(&mut bytes);
            assert!(
                result.is_ok(),
                "Client ID of length {} should be valid",
                length
            );
        }
    }

    #[test]
    fn client_id_length_exceeding_23_bytes_should_return_error() {
        let client_id: String = "A".repeat(24); // 24 bytes
        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, &client_id).unwrap();
        let result = ConnectPayload::parse_client_id(&mut bytes);
        assert!(
            result.is_err(),
            "Client ID exceeding 23 bytes should be invalid"
        );
    }
    // todo extend The Server MAY allow ClientId’s that contain more than 23 encoded bytes. The Server MAY allow ClientId’s that contain characters not included in the list given above.
    #[test]
    fn will_topic_and_will_message_must_be_present_when_will_flag_is_true() {
        let will_flag = true;
        let connect_flags = ConnectFlags::new(false, false, false, 0, will_flag, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);

        let will_topic = "test/will/topic";
        let will_message = "This is a will message";

        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();
        utf::utf_8_handler::write(&mut bytes, will_topic).unwrap();
        utf::utf_8_handler::write(&mut bytes, will_message).unwrap();

        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        let will_topic = payload.will_topic().unwrap();
        let will_message = payload.will_message().unwrap();
        assert_eq!(will_topic, will_topic);
        assert_eq!(will_message, will_message);
    }

    #[test]
    fn will_message_can_set_zero_length_when_will_flag_is_true() {
        let will_flag = true;
        let connect_flags = ConnectFlags::new(false, false, false, 0, will_flag, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);

        let will_topic = "test/will/topic";
        let will_message = "";

        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();
        utf::utf_8_handler::write(&mut bytes, will_topic).unwrap();
        utf::utf_8_handler::write(&mut bytes, will_message).unwrap();

        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        let will_topic = payload.will_topic().unwrap();
        let will_message = payload.will_message().unwrap();
        assert_eq!(will_topic, will_topic);
        assert_eq!(will_message, will_message);
        assert_eq!(will_message.len(), 0);
    }

    #[test]
    fn will_topic_and_will_message_must_be_none_when_will_flag_is_false() {
        let will_flag = false;
        let connect_flags = ConnectFlags::new(false, false, false, 0, will_flag, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);

        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();

        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.will_topic().is_none());
        assert!(payload.will_message().is_none());
    }

    #[test]
    fn will_topic_and_will_message_missing_when_will_flag_is_true_should_return_error() {
        let will_flag = true;
        let connect_flags = ConnectFlags::new(true, false, false, 0, will_flag, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);

        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();

        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_err());
    }

    #[test]
    fn username_must_be_present_when_username_flag_is_true() {
        let username_flag = true;
        let connect_flags =
            ConnectFlags::new(username_flag, false, false, 0, false, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);

        let username = "test_user";

        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();
        utf::utf_8_handler::write(&mut bytes, username).unwrap();

        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        let parsed_username = payload.username().unwrap();
        assert_eq!(parsed_username, username);
    }

    #[test]
    fn username_must_be_none_when_username_flag_is_false() {
        let username_flag = false;
        let connect_flags =
            ConnectFlags::new(username_flag, false, false, 0, false, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);

        let mut bytes = BytesMut::new();

        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();

        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.username().is_none());
    }

    #[test]
    fn username_missing_when_username_flag_is_true_should_return_error() {
        let username_flag = true;
        let connect_flags =
            ConnectFlags::new(username_flag, false, false, 0, false, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);

        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();

        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_err());
    }

    #[test]
    fn username_cannot_set_zero_length_when_username_flag_is_true() {
        let username_flag = true;
        let connect_flags =
            ConnectFlags::new(username_flag, false, false, 0, false, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);

        let username = "";

        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();
        utf::utf_8_handler::write(&mut bytes, username).unwrap();

        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_err());
    }

    #[test]
    fn password_must_be_present_when_password_flag_is_true() {
        let password_flag = true;
        let connect_flags = ConnectFlags::new(true, password_flag, false, 0, false, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);

        let username = "test_user";
        let password = "test_password";

        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();
        utf::utf_8_handler::write(&mut bytes, username).unwrap();
        utf::utf_8_handler::write(&mut bytes, password).unwrap();

        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        let parsed_password = payload.password().unwrap();
        assert_eq!(parsed_password, password);
    }

    #[test]
    fn password_must_be_none_when_password_flag_is_false() {
        let password_flag = false;
        let connect_flags = ConnectFlags::new(true, password_flag, false, 0, false, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);

        let username = "test_user";

        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();
        utf::utf_8_handler::write(&mut bytes, username).unwrap();

        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.password().is_none());
    }

    #[test]
    fn password_missing_when_password_flag_is_true_should_return_error() {
        let password_flag = true;
        let connect_flags = ConnectFlags::new(true, password_flag, false, 0, false, false).unwrap();
        let connect_variable_header = ConnectVariableHeader::new(4, connect_flags, 0);
        let username = "test_user";
        let mut bytes = BytesMut::new();
        utf::utf_8_handler::write(&mut bytes, "Client123").unwrap();
        utf::utf_8_handler::write(&mut bytes, username).unwrap();
        let result = ConnectPayload::parse(&mut bytes, &connect_variable_header);
        assert!(result.is_err());
    }
}
