use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;

pub(crate) struct RemainingLengthParser;

impl RemainingLengthParser {
    pub fn parse(bytes_ops: &mut impl ByteOperations) -> Result<u32, MQTTProtocolError> {
        const MAX_MULTIPLIER: u32 = 128 * 128 * 128;
        let mut value: u32 = 0;
        let mut multiplier: u32 = 1;
        let mut bytes_read = 0;

        loop {
            let current_byte = bytes_ops
                .read_a_byte()
                .ok_or(MQTTProtocolError::PacketTooShort)?;

            bytes_read += 1;

            value += calculate_current_value(current_byte, multiplier);

            if is_end_byte(current_byte) {
                return Ok(value);
            }

            if exceeds_max_multiplier(multiplier) {
                return Err(MQTTProtocolError::MalformedRemainingLength);
            }

            if exceeds_max_bytes(bytes_read) {
                return Err(MQTTProtocolError::MalformedRemainingLength);
            }

            multiplier *= 128;
        }

        fn calculate_current_value(encoded_byte: u8, multiplier: u32) -> u32 {
            (encoded_byte & 0x7F) as u32 * multiplier
        }

        fn is_end_byte(encoded_byte: u8) -> bool {
            (encoded_byte & 0x80) == 0
        }

        fn exceeds_max_multiplier(multiplier: u32) -> bool {
            multiplier > MAX_MULTIPLIER
        }

        fn exceeds_max_bytes(bytes_read: usize) -> bool {
            bytes_read == 4
        }
    }
}

#[cfg(test)]
mod remaining_length_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
    use crate::protocol::mqtt::mqtt4::remaining_length::RemainingLengthParser;
    use bytes::BufMut;

    #[test]
    fn remaining_length_one_byte_is_64() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0b0100_0000);

        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 64);
    }

    #[test]
    fn remaining_length_min_one_byte_is_0() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0x00);

        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 0);
    }

    #[test]
    fn remaining_length_max_one_byte_is_127() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0x7F);

        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 127);
    }

    #[test]
    fn remaining_length_two_bytes_321() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0b11000001);
        bytes_mut.write_a_byte(0b00000010);

        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 321);
    }

    #[test]
    fn remaining_length_min_two_bytes_is_128() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0x80); // 0x80
        bytes_mut.write_a_byte(0x01); // 0x01

        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 128);
    }

    #[test]
    fn remaining_length_max_two_bytes_is_16383() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0xFF); // 0xFF
        bytes_mut.write_a_byte(0x7F); // 0x7F
        

        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 16383);
    }

    #[test]
    fn remaining_length_three_bytes_is_70000() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0b1111_0000); // 0xF0
        bytes_mut.write_a_byte(0b1010_0010); // 0xA2
        bytes_mut.write_a_byte(0b0000_0100); // 0x04
        
        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 70000);
    }

    #[test]
    fn remaining_length_min_three_bytes() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0x80); // 0x80
        bytes_mut.write_a_byte(0x80); // 0x80
        bytes_mut.write_a_byte(0x01); // 0x01
        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 16384);
    }

    #[test]
    fn remaining_length_max_three_bytes() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0xFF); // 0xFF
        bytes_mut.write_a_byte(0xFF); // 0xFF
        bytes_mut.write_a_byte(0x7F); // 0x7F
        
        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 2097151);
    }

    #[test]
    fn remaining_length_four_bytes_is_268435455() {
        // Example: Remaining Length = 268435455
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0b1111_1111); // 0xFF
        bytes_mut.write_a_byte(0b1111_1111); // 0xFF
        bytes_mut.write_a_byte(0b1111_1111); // 0xFF
        bytes_mut.write_a_byte(0b0111_1111); // 0x7F
        
        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 268435455);
    }

    // todo min four bytes
    #[test]
    fn remaining_length_min_four_bytes() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0x80); // 0x80
        bytes_mut.write_a_byte(0x80); // 0x80
        bytes_mut.write_a_byte(0x80); // 0x80
        bytes_mut.write_a_byte(0x01); // 0x01
        
        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 2097152);
    }
    // todo max four bytes
    #[test]
    fn remaining_length_max_four_bytes() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0xFF); // 0xFF
        bytes_mut.write_a_byte(0xFF); // 0xFF
        bytes_mut.write_a_byte(0xFF); // 0xFF
        bytes_mut.write_a_byte(0x7F); // 0x7F
        
        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 268435455);
    }

    // todo if more than 4 bytes are used, then it is an error
    #[test]
    fn remaining_length_more_than_four_bytes_should_error() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0x80); // 0x80
        bytes_mut.write_a_byte(0x80); // 0x80
        bytes_mut.write_a_byte(0x80); // 0x80
        bytes_mut.write_a_byte(0x80); // 0x80
        bytes_mut.write_a_byte(0x01); // 0x01
        
        let result = RemainingLengthParser::parse(&mut bytes_mut);
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            MQTTProtocolError::MalformedRemainingLength
        ));
    }

    #[test]
    fn remaining_length_incomplete_bytes_should_error() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0x80); // 0x80
        // Missing subsequent bytes
        
        let result = RemainingLengthParser::parse(&mut bytes_mut);
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            MQTTProtocolError::PacketTooShort
        ));
    }

    #[test]
    fn remaining_length_can_read_a_byte() {
        let mut bytes_mut = bytes::BytesMut::new();
        bytes_mut.write_a_byte(0x4D);
        bytes_mut.write_a_byte(0x01);
        
        let value = RemainingLengthParser::parse(&mut bytes_mut).unwrap();
        assert_eq!(value, 77);
        assert_eq!(bytes_mut.read_a_byte().unwrap(), 1);
    }
}
