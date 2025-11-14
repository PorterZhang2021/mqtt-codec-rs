use crate::protocol::byte_wrapper::byte_operations::ByteOps;
use crate::protocol::byte_wrapper::network_bytes::NetWorkBytes;
use crate::protocol::mqtt::mqtt4::control_packet_type::ControlPacketType;
use crate::protocol::mqtt::mqtt4::fixed_header_flags::FixedHeaderFlags;

struct FixedHeader {
    control_packet_type: ControlPacketType,
    fixed_header_reserve_flags: FixedHeaderFlags,
    remaining_length: u32,
}

impl FixedHeader {
   /* pub fn new(net_work_bytes: impl ByteOps) -> Self {
    }*/
}


#[cfg(test)]
mod fixed_header_tests {
    use crate::protocol::mqtt::mqtt4::control_packet_type::ControlPacketType;
    use crate::protocol::mqtt::mqtt4::fiexd_header::FixedHeader;
    use crate::protocol::mqtt::mqtt4::fixed_header_flags::FixedHeaderFlags;
}