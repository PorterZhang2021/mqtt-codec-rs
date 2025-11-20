pub(crate) mod utf_8_handler {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::utils::code_error::CodeError;
    use crate::protocol::utils::radix::radix_handler;
    use std::ops::RangeInclusive;

    pub fn read(byte_opts: &mut impl ByteOperations) -> Result<String, CodeError> {
        let utf_8_length = decode_length(byte_opts)?;

        let utf8_string = decode_str(byte_opts, utf_8_length)?;

        Ok(utf8_string)
    }

    pub(super) fn decode_length(
        byte_opts: &mut impl ByteOperations,
    ) -> Result<u16, CodeError> {
        let length_bytes = byte_opts.read_bytes(2);
        let utf_8_length = calculate_mqtt_str_length(length_bytes)?;
        Ok(utf_8_length)
    }

    fn calculate_mqtt_str_length(length_bytes: Vec<u8>) -> Result<u16, CodeError> {
        let utf_8_length = radix_handler::be_bytes_to_u16(length_bytes.as_slice())?;
        Ok(utf_8_length)
    }

    pub(super) fn decode_str(
        byte_opts: &mut impl ByteOperations,
        utf_8_length: u16,
    ) -> Result<String, CodeError> {
        let string_bytes = byte_opts.read_bytes(utf_8_length as usize);

        verify_for_mqtt(&string_bytes)?;

        let utf8_string = decode_utf8(string_bytes)?;
        Ok(utf8_string)
    }

    /// we don't verify 0xD800..=0xDFFF, because rust string already do that
    pub(super) fn verify_for_mqtt(string_bytes: &Vec<u8>) -> Result<(), CodeError> {
        let str = std::str::from_utf8(string_bytes).map_err(|_| CodeError::UTF8DecodingError)?;
        const FORBIDDEN_CHAR_FOR_MQTT: &[RangeInclusive<u32>] = &[
            0x0000..=0x0000, // null
            0x0001..=0x001F, // C0
            0x007F..=0x009F, // C1
            0xFDD0..=0xFDEF, //
            0xFFFE..=0xFFFF, //
        ];

        for char in str.chars() {
            let value = char as u32;
            if value == 0xFEFF {
                continue;
            }
            if FORBIDDEN_CHAR_FOR_MQTT.iter().any(|r| r.contains(&value)) {
                return Err(CodeError::MQTTInvalidCode(value));
            }
        }

        Ok(())
    }

    pub(super) fn decode_utf8(string_bytes: Vec<u8>) -> Result<String, CodeError> {
        let utf8_string =
            String::from_utf8(string_bytes).map_err(|_| CodeError::UTF8DecodingError)?;
        Ok(utf8_string)
    }

    pub(crate) fn write(
        byte_opts: &mut impl ByteOperations,
        input: &str,
    ) -> Result<(), CodeError> {
        let string_bytes = encode_utf8(input);
        verify_for_mqtt(&string_bytes)?;

        let length_bytes = encode_mqtt_length(&string_bytes)?;

        byte_opts.write_bytes(&length_bytes);
        byte_opts.write_bytes(&string_bytes);
        Ok(())
    }

    fn encode_mqtt_length(string_bytes: &Vec<u8>) -> Result<[u8; 2], CodeError> {
        radix_handler::u16_to_be_2_bytes(string_bytes.len())
    }

    pub(super) fn encode_utf8(input: &str) -> Vec<u8> {
        input.as_bytes().to_vec()
    }


}

#[cfg(test)]
mod utf_8_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::utils::code_error::CodeError;
    use crate::protocol::utils::utf::utf_8_handler;
    use bytes::BytesMut;

    #[test]
    fn utf_8_handler_should_read_2_byte_to_calculate_length() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_a_byte(0x00);
        bytes_mut.write_a_byte(0x05);
        let length = utf_8_handler::decode_length(&mut bytes_mut).unwrap();
        assert_eq!(length, 5);
    }

    #[test]
    fn utf_8_handler_should_decode_utf_8_string() {
        let mut bytes_mut = BytesMut::new();
        let except_word = "hello";
        utf_8_handler::write(&mut bytes_mut, except_word).unwrap();

        let utf8_string = utf_8_handler::read(&mut bytes_mut).unwrap();

        assert_eq!(utf8_string, except_word);
    }

    #[test]
    fn utf_8_handler_should_not_allow_u_d800() {
        let invalid_utf8_bytes = vec![0xED, 0xA0, 0x80];
        let result = utf_8_handler::verify_for_mqtt(&invalid_utf8_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CodeError::UTF8DecodingError)))
    }

    #[test]
    fn utf_8_handler_should_not_allow_u_dfff() {
        let invalid_utf8_bytes = vec![0xED, 0xBF, 0xBF];
        let result = utf_8_handler::verify_for_mqtt(&invalid_utf8_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CodeError::UTF8DecodingError)))
    }

    #[test]
    fn utf_8_handler_should_not_allow_u_0000() {
        let invalid_utf8_bytes = vec![0x00];
        let result = utf_8_handler::verify_for_mqtt(&invalid_utf8_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CodeError::MQTTInvalidCode(0x0000))))
    }
    #[test]
    fn utf_8_handler_should_not_allow_u_0001() {
        let invalid_utf8_bytes = vec![0x01];
        let result = utf_8_handler::verify_for_mqtt(&invalid_utf8_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CodeError::MQTTInvalidCode(0x0001))))
    }

    #[test]
    fn utf_8_handler_should_not_allow_u_001f() {
        let invalid_utf8_bytes = vec![0x1F];
        let result = utf_8_handler::verify_for_mqtt(&invalid_utf8_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CodeError::MQTTInvalidCode(0x001F))))
    }

    // todo decode utf-8 can not allowed U+007F to U+009F
    #[test]
    fn utf_8_handler_should_not_allow_u_007f() {
        let invalid_utf8_bytes = vec![0x7F];
        let result = utf_8_handler::verify_for_mqtt(&invalid_utf8_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CodeError::MQTTInvalidCode(0x007F))))
    }

    #[test]
    fn utf_8_handler_should_not_allow_u_009f() {
        let invalid_utf8_bytes = vec![0xC2, 0x9F];
        let result = utf_8_handler::verify_for_mqtt(&invalid_utf8_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CodeError::MQTTInvalidCode(0x009F))))
    }

    // todo decode utf-8 can not allowed U+FDD0 to U+FDEF
    #[test]
    fn utf_8_handler_should_not_allow_u_fdd0() {
        let invalid_utf8_bytes = vec![0xEF, 0xB7, 0x90];
        let result = utf_8_handler::verify_for_mqtt(&invalid_utf8_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CodeError::MQTTInvalidCode(0xFDD0))))
    }

    #[test]
    fn utf_8_handler_should_not_allow_u_fdef() {
        let invalid_utf8_bytes = vec![0xEF, 0xB7, 0xAF];
        let result = utf_8_handler::verify_for_mqtt(&invalid_utf8_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CodeError::MQTTInvalidCode(0xFDEF))))
    }

    #[test]
    fn utf_8_handler_should_not_allow_u_ffff() {
        let invalid_utf8_bytes = vec![0xEF, 0xBF, 0xBF];
        let result = utf_8_handler::verify_for_mqtt(&invalid_utf8_bytes);
        assert!(result.is_err());
        assert!(matches!(result, Err(CodeError::MQTTInvalidCode(0xFFFF))))
    }

    #[test]
    fn utf_8_handler_should_allow_u_feff() {
        let valid_utf8_bytes = vec![0xEF, 0xBB, 0xBF];
        let result = utf_8_handler::verify_for_mqtt(&valid_utf8_bytes);
        assert!(result.is_ok());
    }
}
