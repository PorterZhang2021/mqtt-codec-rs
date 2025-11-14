use bytes::{Buf, BufMut, BytesMut};

pub trait ByteOperations {
    fn read_a_byte(&mut self) -> Option<u8>;
    fn read_bytes(&mut self, len: usize) -> Vec<u8>;
    fn write_a_byte(&mut self, byte: u8);
    fn write_bytes(&mut self, bytes: &[u8]);
    fn bytes_len(&self) -> usize;
    fn get_available_len(&mut self, len: usize) -> usize;
    fn is_empty(&self) -> bool {
        self.bytes_len() == 0
    }
}

impl ByteOperations for BytesMut {
    fn read_a_byte(&mut self) -> Option<u8> {
        if (self.is_empty()) {
            return None;
        }
        Some(self.get_u8())
    }

    fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let available_len = self.get_available_len(len);
        let mut buf = vec![0u8; available_len];
        self.copy_to_slice(&mut buf);
        buf
    }

    fn write_a_byte(&mut self, byte: u8) {
        self.put_u8(byte);
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }

    fn bytes_len(&self) -> usize {
        self.len()
    }

    fn get_available_len(&mut self, len: usize) -> usize {
        let mut available_len = len;
        if (available_len > self.len()) {
            available_len = self.len();
        }
        available_len
    }
}

#[cfg(test)]
mod byte_ops_tests {
    use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
    use bytes::BytesMut;

    #[test]
    fn bytes_mut_can_write_a_byte() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_a_byte(0xAB);
        assert_eq!(bytes_mut.bytes_len(), 1);
    }

    #[test]
    fn bytes_mut_can_write_bytes() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0x01, 0x02, 0x03]);
        assert_eq!(bytes_mut.bytes_len(), 3);
    }

    #[test]
    fn bytes_mut_can_read_a_byte() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_a_byte(0xCD);
        assert_eq!(bytes_mut.bytes_len(), 1);
        let byte = bytes_mut.read_a_byte();
        assert_eq!(byte, Some(0xCD));
    }

    #[test]
    fn bytes_must_can_read_bytes() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0x01, 0x02, 0x03]);
        assert_eq!(bytes_mut.bytes_len(), 3);
        let bytes = bytes_mut.read_bytes(3);
        assert_eq!(bytes, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn bytes_mut_bytes_len_should_return_correct_length() {
        let mut bytes_mut = BytesMut::new();
        assert_eq!(bytes_mut.bytes_len(), 0);
        bytes_mut.write_bytes(&[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(bytes_mut.bytes_len(), 4);
    }

    #[test]
    fn bytes_mut_read_more_than_available_bytes() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0x01, 0x02]);
        assert_eq!(bytes_mut.bytes_len(), 2);
        let bytes = bytes_mut.read_bytes(3);
        assert_eq!(bytes, vec![0x01, 0x02]);
    }

    #[test]
    fn bytes_mut_read_a_byte_from_empty_bytes_mut_should_return_none() {
        let mut bytes_mut = BytesMut::new();
        let byte = bytes_mut.read_a_byte();
        assert!(byte.is_none());
        assert_eq!(byte, None);
    }

    #[test]
    fn byte_mut_read_bytes_from_empty_bytes_mut_should_return_empty_vec() {
        let mut bytes_mut = BytesMut::new();
        let bytes = bytes_mut.read_bytes(5);
        assert!(bytes.is_empty());
        assert_eq!(bytes, vec![]);
    }

    #[test]
    fn bytes_mut_get_available_len_should_return_correct_length() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_bytes(&[0x01, 0x02, 0x03]);
        assert_eq!(bytes_mut.get_available_len(2), 2);
        assert_eq!(bytes_mut.get_available_len(5), 3);
    }
}
