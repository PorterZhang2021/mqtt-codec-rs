use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::utf;

struct PublishPayload {
    application_message: String,
}

impl PublishPayload {
    pub fn application_message(&self) -> &str {
        &self.application_message
    }
}

impl PublishPayload {
    pub(crate) fn parse(
        bytes: &mut impl ByteOperations,
    ) -> Result<PublishPayload, MQTTProtocolError> {
        let application_message = Self::parse_application_message(bytes)?;
        Ok(PublishPayload {
            application_message,
        })
    }

    fn parse_application_message(
        bytes: &mut impl ByteOperations,
    ) -> Result<String, MQTTProtocolError> {
        let application_message = utf::utf_8_handler::read(bytes)?;
        Ok(application_message)
    }
}

#[cfg(test)]
mod publish_payload_tests {
    use crate::protocol::mqtt::mqtt4::payload_parser::publish::PublishPayload;
    use crate::protocol::utils::utf::utf_8_handler::write;
    use bytes::BytesMut;

    #[test]
    fn publish_payload_parser_should_parse_payload_correctly() {
        let mut bytes = BytesMut::new();
        write(&mut bytes, "Hello MQTT");

        let publish_payload = PublishPayload::parse(&mut bytes).unwrap();

        assert_eq!(publish_payload.application_message(), "Hello MQTT");
    }

    #[test]
    fn publish_payload_can_handle_empty_message() {
        let mut bytes = BytesMut::new();
        write(&mut bytes, "");

        let publish_payload = PublishPayload::parse(&mut bytes).unwrap();

        assert_eq!(publish_payload.application_message(), "");
    }
}
