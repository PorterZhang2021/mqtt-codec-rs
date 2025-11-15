pub(crate) mod binary_handler {
    use crate::protocol::utils::code_error::CodeError;

    #[inline]
    pub(crate) fn low_nibble(binary_byte: u8) -> u8 {
        binary_byte & 0x0F
    }

    #[inline]
    pub(crate) fn high_nibble(binary_byte: u8) -> u8 {
        binary_byte >> 4
    }

    #[inline]
    pub(crate) fn binary_byte_to_decimal(binary_byte: u8) -> u8 {
        binary_byte
    }

    #[inline]
    pub(crate) fn decimal_to_binary_byte(decimal_value: u8) -> u8 {
        decimal_value
    }

    #[inline]
    pub(crate) fn be_bytes_to_u16(bytes: &[u8]) -> Result<u16, CodeError> {
        bytes
            .get(..2)
            .and_then(|slice| slice.try_into().ok())
            .map(u16::from_be_bytes)
            .ok_or(CodeError::CodeLengthError(2, bytes.len()))
    }
}

#[cfg(test)]
mod binary_util_test {
    use crate::protocol::utils::code_error::CodeError;
    use crate::protocol::utils::radix::binary_handler;

    #[test]
    fn binary_high_4bits_should_convert_to_8bits() {
        let value: u8 = 0b0000_0101;
        let binary_high_4bits = binary_handler::high_nibble(value);
        assert_eq!(binary_high_4bits, 0b0000_0000);
    }

    #[test]
    fn binary_low_4bits_should_convert_to_8bits() {
        let value: u8 = 0b1010_1111;
        let binary_low_4bits = binary_handler::low_nibble(value);
        assert_eq!(binary_low_4bits, 0b0000_1111);
    }

    #[test]
    fn binary_8bits_should_convert_to_decimal() {
        let value: u8 = 0b0000_1010;
        let value = binary_handler::binary_byte_to_decimal(value);
        assert_eq!(value, 10);
    }

    #[test]
    fn decimal_should_convert_to_binary_8bits() {
        let value: u8 = 10;
        let value = binary_handler::decimal_to_binary_byte(value);
        assert_eq!(value, 0b0000_1010);
    }

    #[test]
    fn be_bytes_should_convert_to_u16() {
        let bytes: Vec<u8> = vec![0x12, 0x34];
        let value = binary_handler::be_bytes_to_u16(&bytes).unwrap();
        assert_eq!(value, 0x1234);
    }

    #[test]
    fn be_bytes_to_u16_should_return_error_when_length_invalid() {
        let bytes: Vec<u8> = vec![0x12];
        let result = binary_handler::be_bytes_to_u16(&bytes);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CodeError::CodeLengthError(2, 1))
        ));
    }
}
