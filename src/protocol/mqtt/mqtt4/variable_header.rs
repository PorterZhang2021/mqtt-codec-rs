#[cfg(test)]
mod variable_header_tests {
    use bytes::BytesMut;
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;

    // todo connect
    // todo connect contain protocol name
    #[test]
    fn connect_contain_protocol_name() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0x00, 0x04, b'M', b'Q', b'T', b'T']);
        let length = bytes_mut.read_bytes(2);
        // let protocol_name = VariableHeader::parse_connect_protocol_name(&mut bytes_mut).unwrap();
        // assert_eq!(protocol_name, "MQTT");
    }
    // todo connect contain protocol level
    // todo connect contain connect flags
    // todo connect keep alive
    // todo connack
    // todo publish
    // todo puback
    // todo pubrec
    // todo pubrel
    // todo pubcomp
    // todo subscribe
    // todo suback
    // todo unsubscribe
    // todo unsuback
    // todo pingreq
    // todo pingresp
    // todo disconnect
}