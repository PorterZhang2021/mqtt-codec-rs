pub(crate) mod utf_8_handler {

}

#[cfg(test)]
mod utf_8_tests {
    use bytes::BytesMut;
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use crate::protocol::utils::radix::binary_handler;

    // todo read 2 byte to get length
    #[test]
    fn utf_8_handler_should_read_2_byte_to_calculate_length() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_a_byte(0x00);
        bytes_mut.write_a_byte(0x05);
        let vec = bytes_mut.read_bytes(2);
       // let length = binary_handler::be_bytes_to_u16(vec);
       // assert_eq!(length, 5);
    }
    // todo read 2 byte to get length then decode utf-8 string

    // this case need filter invalid utf-8
    // todo decode utf-8 can not allowed U+D800 to U+DFFF
    // todo decode utf-8 can not allowed U+0000
    // todo decode utf-8 can not allowed U+0001 to U+001F
    // todo decode utf-8 can not allowed U+007F to U+009F
    // todo decode utf-8 can not allowed U+FDD0 to U+FDEF
    // todo decode utf-8 receive 0XEF 0XBB 0XBF to U+EFFF
}