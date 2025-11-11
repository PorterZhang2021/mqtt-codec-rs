pub(crate) struct BinaryUtils;

impl BinaryUtils {
    pub(crate) fn binary_low_4bits_to_8bits(binary_byte: u8) -> u8 {
        binary_byte & 0b0000_1111
    }

    pub(crate) fn binary_high_4bits_to_8bits(binary_byte: u8) -> u8 {
        binary_byte >> 4
    }

    pub(crate) fn binary_8bits_convert_to_decimal(binary_byte: u8) -> u8 {
        binary_byte
    }

    pub(crate) fn decimal_convert_to_binary_8bits(decimal_value: u8) -> u8 {
        decimal_value
    }
}

#[cfg(test)]
mod binary_util_test {
    use crate::protocol::utils::radix::BinaryUtils;

    #[test]
    fn binary_high_4bits_should_convert_to_8bits() {
        let value: u8 = 0b0000_0101;
        let binary_high_4bits = BinaryUtils::binary_high_4bits_to_8bits(value);
        assert_eq!(binary_high_4bits, 0b0000_0000);
    }

    #[test]
    fn binary_low_4bits_should_convert_to_8bits() {
        let value: u8 = 0b1010_1111;
        let binary_low_4bits = BinaryUtils::binary_low_4bits_to_8bits(value);
        assert_eq!(binary_low_4bits, 0b0000_1111);
    }

    #[test]
    fn binary_8bits_should_convert_to_decimal() {
        let value: u8 = 0b0000_1010;
        let value = BinaryUtils::binary_8bits_convert_to_decimal(value);
        assert_eq!(value, 10);
    }

    #[test]
    fn decimal_should_convert_to_binary_8bits() {
        let value: u8 = 10;
        let value = BinaryUtils::decimal_convert_to_binary_8bits(value);
        assert_eq!(value, 0b0000_1010);
    }
    
    #[test]
    fn binary_first_bits_can_get() {
        todo!()
    }
}
