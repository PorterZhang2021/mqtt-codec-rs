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

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConnectPayload {
    client_id: String,
    will_topic: Option<String>,
    will_message: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[allow(dead_code)]
impl ConnectPayload {
    pub fn new(
        client_id: String,
        will_topic: Option<String>,
        will_message: Option<String>,
        username: Option<String>,
        password: Option<String>,
    ) -> Self {
        ConnectPayload {
            client_id,
            will_topic,
            will_message,
            username,
            password,
        }
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn will_topic(&self) -> Option<&str> {
        self.will_topic.as_deref()
    }

    pub fn will_message(&self) -> Option<&str> {
        self.will_message.as_deref()
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }
}

#[cfg(test)]
mod connect_payload_decode_tests {
    use crate::protocol::common::protocol_level::ProtocolLevel;
    use crate::protocol::common::qos::QoSCode;
    use crate::protocol::mqtt_3_1_1::payload_parser::connect_parser::payload::ConnectPayload;
    use crate::protocol::mqtt_3_1_1::payload_parser::payload_codec::PayloadEncoder;
    use crate::protocol::mqtt_3_1_1::variable_header_parser::connect_parser::variable_header::{
        ConnectFlags, ConnectVariableHeader,
    };
    use bytes::BytesMut;

    #[test]
    fn client_id_should_contain_only_valid_characters() {
        let valid_client_id = "Client123";
        let connect_payload =
            ConnectPayload::new(valid_client_id.to_string(), None, None, None, None);
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);
        let result = ConnectPayload::parse_client_id(&mut bytes).unwrap();
        assert_eq!(result, valid_client_id);
    }

    #[test]
    fn client_id_with_invalid_characters_should_return_error() {
        let invalid_client_id = "Client@123";
        let connect_payload =
            ConnectPayload::new(invalid_client_id.to_string(), None, None, None, None);
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::parse_client_id(&mut bytes);
        assert!(result.is_err());
    }

    #[test]
    fn client_id_can_be_zero_length_if_clean_session_is_true() {
        let clean_session = true;
        let connect_flags =
            ConnectFlags::new(false, false, false, QoSCode::Qos0, false, clean_session).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);
        let expect_client_id = "";
        let connect_payload =
            ConnectPayload::new(expect_client_id.to_string(), None, None, None, None);
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);
        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert_eq!(payload.client_id(), expect_client_id);
    }

    #[test]
    fn client_id_cannot_be_zero_length_if_clean_session_is_false() {
        let clean_session = false;
        let connect_flags =
            ConnectFlags::new(false, false, false, QoSCode::Qos0, false, clean_session).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);
        let expect_client_id = "";
        let connect_payload =
            ConnectPayload::new(expect_client_id.to_string(), None, None, None, None);
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);
        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_err());
    }

    #[test]
    fn client_id_length_between_1_and_23_bytes() {
        for length in 1..=23 {
            let expect_client_id: String = "A".repeat(length);

            let connect_payload =
                ConnectPayload::new(expect_client_id.to_string(), None, None, None, None);
            let vec = connect_payload.encode().unwrap();
            let mut bytes = BytesMut::from(&vec[..]);

            let result = ConnectPayload::parse_client_id(&mut bytes);
            assert!(
                result.is_ok(),
                "Client ID of length {} should be valid",
                length
            );
            assert_eq!(result.unwrap(), expect_client_id);
        }
    }

    #[test]
    fn client_id_length_exceeding_23_bytes_should_return_error() {
        let client_id: String = "A".repeat(24); // 24 bytes

        let connect_payload = ConnectPayload::new(client_id.to_string(), None, None, None, None);
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::parse_client_id(&mut bytes);
        assert!(
            result.is_err(),
            "Client ID exceeding 23 bytes should be invalid"
        );
    }

    // todo extend The Server MAY allow ClientId’s that contain more than 23 encoded bytes.
    // The Server MAY allow ClientId’s that contain characters not included in the list given above.
    #[test]
    fn will_topic_and_will_message_must_be_present_when_will_flag_is_true() {
        let will_flag = true;
        let connect_flags =
            ConnectFlags::new(false, false, false, QoSCode::Qos0, will_flag, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let client_id = "Client123";
        let expect_will_topic = "test/will/topic";
        let expect_will_message = "This is a will message";

        let connect_payload = ConnectPayload::new(
            client_id.to_string(),
            Some(expect_will_topic.to_string()),
            Some(expect_will_message.to_string()),
            None,
            None,
        );
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert_eq!(payload.will_topic().unwrap(), expect_will_topic);
        assert_eq!(payload.will_message().unwrap(), expect_will_message);
    }

    #[test]
    fn will_message_can_set_zero_length_when_will_flag_is_true() {
        let will_flag = true;
        let connect_flags =
            ConnectFlags::new(false, false, false, QoSCode::Qos0, will_flag, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let expect_client_id = "Client123";
        let expect_will_topic = "test/will/topic";
        let expect_will_message = "";

        let connect_payload = ConnectPayload::new(
            expect_client_id.to_string(),
            Some(expect_will_topic.to_string()),
            Some(expect_will_message.to_string()),
            None,
            None,
        );
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert_eq!(payload.client_id(), expect_client_id);
        assert_eq!(payload.will_topic().unwrap(), expect_will_topic);
        assert_eq!(payload.will_message().unwrap(), expect_will_message);
    }

    #[test]
    fn will_topic_and_will_message_must_be_none_when_will_flag_is_false() {
        let will_flag = false;
        let connect_flags =
            ConnectFlags::new(false, false, false, QoSCode::Qos0, will_flag, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let expect_client_id = "Client123";
        let connect_payload =
            ConnectPayload::new(expect_client_id.to_string(), None, None, None, None);
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert_eq!(payload.client_id(), expect_client_id);
        assert!(payload.will_topic().is_none());
        assert!(payload.will_message().is_none());
    }

    #[test]
    fn will_topic_and_will_message_missing_when_will_flag_is_true_should_return_error() {
        let will_flag = true;
        let connect_flags =
            ConnectFlags::new(true, false, false, QoSCode::Qos0, will_flag, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let client_id = "Client123";
        let connect_payload = ConnectPayload::new(client_id.to_string(), None, None, None, None);
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_err());
    }

    #[test]
    fn username_must_be_present_when_username_flag_is_true() {
        let username_flag = true;
        let connect_flags =
            ConnectFlags::new(username_flag, false, false, QoSCode::Qos0, false, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let expect_client_id = "Client123";
        let expect_username = "test_user";

        let connect_payload = ConnectPayload::new(
            expect_client_id.to_string(),
            None,
            None,
            Some(expect_username.to_string()),
            None,
        );
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert_eq!(payload.client_id(), expect_client_id);
        assert_eq!(payload.username().unwrap(), expect_username);
    }

    #[test]
    fn username_must_be_none_when_username_flag_is_false() {
        let username_flag = false;
        let connect_flags =
            ConnectFlags::new(username_flag, false, false, QoSCode::Qos0, false, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let expect_client_id = "Client123";
        let connect_payload =
            ConnectPayload::new(expect_client_id.to_string(), None, None, None, None);
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert_eq!(expect_client_id, payload.client_id());
        assert!(payload.username().is_none());
    }

    #[test]
    fn username_missing_when_username_flag_is_true_should_return_error() {
        let username_flag = true;
        let connect_flags =
            ConnectFlags::new(username_flag, false, false, QoSCode::Qos0, false, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let expect_client_id = "Client123";
        let connect_payload =
            ConnectPayload::new(expect_client_id.to_string(), None, None, None, None);
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_err());
    }

    #[test]
    fn username_cannot_set_zero_length_when_username_flag_is_true() {
        let username_flag = true;
        let connect_flags =
            ConnectFlags::new(username_flag, false, false, QoSCode::Qos0, false, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let expect_client_id = "Client123";
        let expect_username = "";

        let connect_payload = ConnectPayload::new(
            expect_client_id.to_string(),
            None,
            None,
            Some(expect_username.to_string()),
            None,
        );
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_err());
    }

    #[test]
    fn password_must_be_present_when_password_flag_is_true() {
        let password_flag = true;
        let connect_flags =
            ConnectFlags::new(true, password_flag, false, QoSCode::Qos0, false, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let expect_client_id = "Client123";
        let expect_username = "test_user";
        let expect_password = "test_password";

        let connect_payload = ConnectPayload::new(
            expect_client_id.to_string(),
            None,
            None,
            Some(expect_username.to_string()),
            Some(expect_password.to_string()),
        );
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert_eq!(payload.client_id(), expect_client_id);
        assert_eq!(payload.username().unwrap(), expect_username);
        assert_eq!(payload.password().unwrap(), expect_password);
    }

    #[test]
    fn password_must_be_none_when_password_flag_is_false() {
        let password_flag = false;
        let connect_flags =
            ConnectFlags::new(true, password_flag, false, QoSCode::Qos0, false, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let expect_client_id = "Client123";
        let expect_username = "test_user";

        let connect_payload = ConnectPayload::new(
            expect_client_id.to_string(),
            None,
            None,
            Some(expect_username.to_string()),
            None,
        );
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert_eq!(payload.client_id(), expect_client_id);
        assert_eq!(payload.username().unwrap(), expect_username);
        assert!(payload.password().is_none());
    }

    #[test]
    fn password_missing_when_password_flag_is_true_should_return_error() {
        let password_flag = true;
        let connect_flags =
            ConnectFlags::new(true, password_flag, false, QoSCode::Qos0, false, false).unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);
        let expect_client_id = "Client123";
        let expect_username = "test_user";

        let connect_payload = ConnectPayload::new(
            expect_client_id.to_string(),
            None,
            None,
            Some(expect_username.to_string()),
            None,
        );
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_err());
    }

    #[test]
    fn complete_payload_encoding_and_decoding() {
        let will_flag = true;
        let username_flag = true;
        let password_flag = true;
        let connect_flags = ConnectFlags::new(
            username_flag,
            password_flag,
            false,
            QoSCode::Qos0,
            will_flag,
            false,
        )
        .unwrap();
        let connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, connect_flags, 0);

        let expect_client_id = "Client123";
        let expect_will_topic = "test/will/topic";
        let expect_will_message = "This is a will message";
        let expect_username = "test_user";
        let expect_password = "test_password";

        let connect_payload = ConnectPayload::new(
            expect_client_id.to_string(),
            Some(expect_will_topic.to_string()),
            Some(expect_will_message.to_string()),
            Some(expect_username.to_string()),
            Some(expect_password.to_string()),
        );
        let vec = connect_payload.encode().unwrap();
        let mut bytes = BytesMut::from(&vec[..]);

        let result = ConnectPayload::decode(&mut bytes, &connect_variable_header);
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert_eq!(payload.client_id(), expect_client_id);
        assert_eq!(payload.will_topic().unwrap(), expect_will_topic);
        assert_eq!(payload.will_message().unwrap(), expect_will_message);
        assert_eq!(payload.username().unwrap(), expect_username);
        assert_eq!(payload.password().unwrap(), expect_password);
    }
}
