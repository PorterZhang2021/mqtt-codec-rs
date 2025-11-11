use crate::protocol::byte_wrapper::byte_operations::ByteOps;
use bytes::{Buf, BufMut, BytesMut};

pub struct NetWorkBytes<T> {
    inner: T,
}
impl<T: ByteOps> ByteOps for NetWorkBytes<T> {
    fn read_a_byte(&mut self) -> Option<u8> {
        self.inner.read_a_byte()
    }

    fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        self.inner.read_bytes(len)
    }

    fn write_a_byte(&mut self, byte: u8) {
        self.inner.write_a_byte(byte)
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        self.inner.write_bytes(bytes)
    }

    fn bytes_len(&self) -> usize {
        self.inner.bytes_len()
    }

    fn get_available_len(&mut self, len: usize) -> usize {
        self.inner.get_available_len(len)
    }
}

impl<T> NetWorkBytes<T> {
    pub fn new(inner: T) -> Self {
        NetWorkBytes { inner }
    }
}

#[cfg(test)]
mod network_bytes_tests {
    use super::*;

    #[test]
    fn network_bytes_can_write_a_byte() {
        let bytes_mut = BytesMut::new();
        let mut network_bytes = NetWorkBytes::new(bytes_mut);
        network_bytes.write_a_byte(0xAB);
        assert_eq!(network_bytes.bytes_len(), 1);
    }

    #[test]
    fn network_bytes_can_write_bytes() {
        let bytes_mut = BytesMut::new();
        let mut network_bytes = NetWorkBytes::new(bytes_mut);
        network_bytes.write_bytes(&[0x01, 0x02, 0x03]);
        assert_eq!(network_bytes.bytes_len(), 3);
    }

    #[test]
    fn network_bytes_can_read_a_byte() {
        let mut bytes_mut = BytesMut::new();
        let mut network_bytes = NetWorkBytes::new(bytes_mut);
        network_bytes.write_a_byte(0xCD);
        let byte = network_bytes.read_a_byte();
        assert_eq!(byte, Some(0xCD));
    }

    #[test]
    fn network_bytes_can_read_bytes() {
        let mut bytes_mut = BytesMut::new();
        let mut network_bytes = NetWorkBytes::new(bytes_mut);
        network_bytes.write_bytes(&[0x04, 0x05, 0x06]);
        let bytes = network_bytes.read_bytes(3);
        assert_eq!(bytes, vec![0x04, 0x05, 0x06]);
    }
}
