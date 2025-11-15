use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::mqtt::mqtt4::control_packet_type::ControlPacketType;
use crate::protocol::mqtt::mqtt4::fixed_header_flags::FixedHeaderFlags;
use crate::protocol::mqtt::mqtt4::remaining_length::RemainingLengthParser;

pub struct FixedHeader {
    control_packet_type: ControlPacketType,
    fixed_header_reserve_flags: FixedHeaderFlags,
    remaining_length: u32,
}

impl FixedHeader {
    pub fn parse(bytes: &mut impl ByteOperations) -> Result<FixedHeader, MQTTProtocolError> {
        let first_byte = bytes
            .read_a_byte()
            .ok_or(MQTTProtocolError::PacketTooShort)?;
        let control_packet_type = ControlPacketType::parse(first_byte)?;
        let fixed_header_reserve_flags =
            FixedHeaderFlags::parse(control_packet_type.clone(), first_byte)?;

        let remaining_length = RemainingLengthParser::parse(bytes)?;

        Ok(FixedHeader {
            control_packet_type,
            fixed_header_reserve_flags,
            remaining_length,
        })
    }
}

#[cfg(test)]
mod fixed_header_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
    use crate::protocol::mqtt::mqtt4::control_packet_type::ControlPacketType;
    use crate::protocol::mqtt::mqtt4::fixed_header::FixedHeader;
    use crate::protocol::mqtt::mqtt4::fixed_header_flags::FixedHeaderFlags;
    use bytes::BytesMut;

    #[test]
    fn fixed_header_can_parse_connect_packet() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0b0001_0000, 0b0000_0010, 0b0000_0100]);
        let fixed_header = FixedHeader::parse(&mut bytes_mut).unwrap();
        assert_eq!(fixed_header.control_packet_type, ControlPacketType::Connect);
        assert_eq!(
            fixed_header.fixed_header_reserve_flags,
            FixedHeaderFlags::Connect
        );
        assert_eq!(fixed_header.remaining_length, 2);
        assert_eq!(bytes_mut.read_a_byte().unwrap(), 0b0000_0100);
    }

    #[test]
    fn fixed_header_can_parse_publish_packet() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0b0011_1101, 0b0000_0011, 0b0000_0101, 0b0000_0110]);
        let fixed_header = FixedHeader::parse(&mut bytes_mut).unwrap();
        assert_eq!(fixed_header.control_packet_type, ControlPacketType::Publish);
        assert_eq!(
            fixed_header.fixed_header_reserve_flags,
            FixedHeaderFlags::Publish {
                dup: true,
                qos: 2,
                retain: true
            }
        );
        assert_eq!(fixed_header.remaining_length, 3);
        assert_eq!(bytes_mut.read_a_byte().unwrap(), 0b0000_0101);
    }

    #[test]
    fn fixe_header_parse_fails_on_short_packet() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0b0001_0000]);
        let result = FixedHeader::parse(&mut bytes_mut);
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            MQTTProtocolError::PacketTooShort
        ));
    }
}
