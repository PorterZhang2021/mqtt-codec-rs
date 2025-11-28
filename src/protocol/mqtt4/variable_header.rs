// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::byte_adapter::byte_operations::ByteOperations;
use crate::protocol::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::mqtt4::control_packet_type::ControlPacketType;
use crate::protocol::mqtt4::fixed_header::FixedHeader;
use crate::protocol::mqtt4::fixed_header_flags::FixedHeaderFlags;
use crate::protocol::mqtt4::variable_header_parser::conn_ack::ConnAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::connect::ConnectVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_ack::PubAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_comp::PubCompVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_rec::PubRecVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_rel::PubRelVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::publish::PublishVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::sub_ack::SubAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::subscribe::SubScribeVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::unsub_ack::UnSubAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::unsubscribe::UnSubScribeVariableHeader;

#[allow(dead_code)]
pub enum VariableHeader {
    Connect {
        connect_variable_header: ConnectVariableHeader,
    },
    ConnAck {
        conn_ack_variable_header: ConnAckVariableHeader,
    },
    Publish {
        publish_variable_header: PublishVariableHeader,
    },
    PubAck {
        pub_ack_variable_header: PubAckVariableHeader,
    },
    PubRec {
        pub_rec_variable_header: PubRecVariableHeader,
    },
    PubRel {
        pub_rel_variable_header: PubRelVariableHeader,
    },
    PubComp {
        pub_comp_variable_header: PubCompVariableHeader,
    },
    Subscribe {
        subscribe_variable_header: SubScribeVariableHeader,
    },
    SubAck {
        sub_ack_variable_header: SubAckVariableHeader,
    },
    Unsubscribe {
        unsubscribe_variable_header: UnSubScribeVariableHeader,
    },
    UnsubAck {
        unsub_ack_variable_header: UnSubAckVariableHeader,
    },
    PingReq,
    PingResp,
    Disconnect,
}

#[allow(dead_code)]
impl VariableHeader {
    fn parse(
        fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<VariableHeader, MQTTProtocolError> {
        match fixed_header.control_packet_type() {
            ControlPacketType::Connect => Ok(VariableHeader::Connect {
                connect_variable_header: ConnectVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::ConnAck => Ok(VariableHeader::ConnAck {
                conn_ack_variable_header: ConnAckVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::Publish => {
                if let FixedHeaderFlags::Publish {
                    dup: _dup,
                    qos,
                    retain: _retain,
                } = fixed_header.fixed_header_reserved_flags()
                {
                    Ok(VariableHeader::Publish {
                        publish_variable_header: PublishVariableHeader::parse(bytes, *qos)?,
                    })
                } else {
                    Err(MQTTProtocolError::MalformedPacket)
                }
            }
            ControlPacketType::PubAck => Ok(VariableHeader::PubAck {
                pub_ack_variable_header: PubAckVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::PubRec => Ok(VariableHeader::PubRec {
                pub_rec_variable_header: PubRecVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::PubRel => Ok(VariableHeader::PubRel {
                pub_rel_variable_header: PubRelVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::PubComp => Ok(VariableHeader::PubComp {
                pub_comp_variable_header: PubCompVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::Subscribe => Ok(VariableHeader::Subscribe {
                subscribe_variable_header: SubScribeVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::SubAck => Ok(VariableHeader::SubAck {
                sub_ack_variable_header: SubAckVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::Unsubscribe => Ok(VariableHeader::Unsubscribe {
                unsubscribe_variable_header: UnSubScribeVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::UnsubAck => Ok(VariableHeader::UnsubAck {
                unsub_ack_variable_header: UnSubAckVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::PingReq => Ok(VariableHeader::PingReq),
            ControlPacketType::PingResp => Ok(VariableHeader::PingResp),
            ControlPacketType::Disconnect => Ok(VariableHeader::Disconnect),
        }
    }
}

#[cfg(test)]
mod variable_header_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::mqtt4::control_packet_type::ControlPacketType;
    use crate::protocol::mqtt4::fixed_header::FixedHeader;
    use crate::protocol::mqtt4::fixed_header_flags::FixedHeaderFlags;
    use crate::protocol::mqtt4::variable_header::VariableHeader;
    use crate::utils::utf::utf_8_handler::write;
    use bytes::BytesMut;

    // todo connect
    #[test]
    fn variable_header_can_parse_connect_packet() {
        let fixed_header =
            FixedHeader::new(ControlPacketType::Connect, FixedHeaderFlags::Connect, 10);

        let mut bytes_mut = BytesMut::new();
        write(&mut bytes_mut, "MQTT").unwrap();
        bytes_mut.write_a_byte(0b0000_0100); // protocol level 4
        bytes_mut.write_a_byte(0b1100_1110); // connect flags
        bytes_mut.write_a_byte(0x00); // keep alive MSB
        bytes_mut.write_a_byte(0x3C); // keep alive LSB (60 seconds)

        let variable_header = VariableHeader::parse(&fixed_header, &mut bytes_mut).unwrap();

        match variable_header {
            VariableHeader::Connect {
                connect_variable_header,
            } => {
                assert_eq!(connect_variable_header.protocol_level, 4);
                assert!(connect_variable_header.connect_flags().username_flag);
                assert!(connect_variable_header.connect_flags().password_flag);
                assert!(!connect_variable_header.connect_flags().will_retain);
                assert_eq!(connect_variable_header.connect_flags().will_qos, 1);
                assert!(connect_variable_header.connect_flags().will_flag);
                assert!(connect_variable_header.connect_flags().clean_session);
                assert_eq!(connect_variable_header.keep_alive, 60);
            }
            _ => panic!("Expected Connect Variable Header"),
        }
    }
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
