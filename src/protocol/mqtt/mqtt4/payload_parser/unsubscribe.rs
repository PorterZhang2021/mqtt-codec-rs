use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::utils::utf;

struct UnSubscribePayload {
    topics: Vec<String>,
}

impl UnSubscribePayload {
    pub(crate) fn parse(
        bytes: &mut impl ByteOperations,
    ) -> Result<UnSubscribePayload, MQTTProtocolError> {
        let mut topics = Vec::new();

        while let Some(topic) = Self::parse_topic(bytes)? {
            topics.push(topic);
        }

        Self::verify_topics_is_empty(&mut topics)?;

        Ok(UnSubscribePayload { topics })
    }

    fn parse_topic(bytes: &mut impl ByteOperations) -> Result<Option<String>, MQTTProtocolError> {
        if bytes.is_empty() {
            return Ok(None);
        }

        let topic_filter = Self::parse_topic_filter(bytes)?;

        Ok(Some(topic_filter))
    }

    fn parse_topic_filter(bytes: &mut impl ByteOperations) -> Result<String, MQTTProtocolError> {
        let topic_filter = utf::utf_8_handler::read(bytes)?;
        Ok(topic_filter)
    }

    fn verify_topics_is_empty(topics: &mut Vec<String>) -> Result<(), MQTTProtocolError> {
        if topics.is_empty() {
            return Err(MQTTProtocolError::MalformedPacket);
        }
        Ok(())
    }
}

#[cfg(test)]
mod unsubscribe_payload_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
    use crate::protocol::mqtt::mqtt4::payload_parser::unsubscribe::UnSubscribePayload;
    use crate::protocol::utils::utf::utf_8_handler::write;
    use bytes::BytesMut;

    #[test]
    fn unsubscribe_payload_can_parse_a_topic() {
        let mut bytes = BytesMut::new();
        write(&mut bytes, "test/topic");

        let unsubscribe_payload = UnSubscribePayload::parse(&mut bytes).unwrap();

        assert_eq!(unsubscribe_payload.topics.len(), 1);
        assert_eq!(unsubscribe_payload.topics[0], "test/topic");
    }

    #[test]
    fn unsubscribe_payload_can_parse_multiple_topics() {
        let mut bytes = BytesMut::new();
        write(&mut bytes, "topic/one");
        write(&mut bytes, "topic/two");
        let unsubscribe_payload = UnSubscribePayload::parse(&mut bytes).unwrap();
        assert_eq!(unsubscribe_payload.topics.len(), 2);
        assert_eq!(unsubscribe_payload.topics[0], "topic/one");
        assert_eq!(unsubscribe_payload.topics[1], "topic/two");
    }

    #[test]
    fn unsubscribe_payload_fails_on_empty_packet() {
        let mut bytes = BytesMut::new();
        let result = UnSubscribePayload::parse(&mut bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(MQTTProtocolError::MalformedPacket)));
    }

    #[test]
    fn unsubscribe_payload_can_handle_wildcard_topics() {
        let mut bytes = BytesMut::new();
        write(&mut bytes, "home/+/temperature");
        write(&mut bytes, "sensors/#");
        let unsubscribe_payload = UnSubscribePayload::parse(&mut bytes).unwrap();
        assert_eq!(unsubscribe_payload.topics.len(), 2);
        assert_eq!(unsubscribe_payload.topics[0], "home/+/temperature");
        assert_eq!(unsubscribe_payload.topics[1], "sensors/#");
    }
}
