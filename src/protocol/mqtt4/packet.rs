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
use crate::protocol::codec::Decoder;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::protocol::mqtt4::control_packet_type::ControlPacketType;
use crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt4::fixed_header_parser::fixed_header_codec::MqttFixedHeaderCodec;
use crate::protocol::mqtt4::payload_parser::connect_parser::payload::ConnectPayload;
use crate::protocol::mqtt4::payload_parser::mqtt_payload_codec::MqttPayloadDecoder;
use crate::protocol::mqtt4::payload_parser::publish_parser::payload::PublishPayload;
use crate::protocol::mqtt4::payload_parser::sub_ack_parser::payload::SubAckPayload;
use crate::protocol::mqtt4::payload_parser::subscribe_parser::payload::SubscribePayload;
use crate::protocol::mqtt4::payload_parser::unsubscribe_parser::payload::UnSubscribePayload;
use crate::protocol::mqtt4::variable_header_parser::conn_ack_parser::variable_header::ConnAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::connect_parser::variable_header::ConnectVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderDecoder;
use crate::protocol::mqtt4::variable_header_parser::pub_ack_parser::variable_header::PubAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_comp::PubCompVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_rec::PubRecVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_rel_parser::variable_header::PubRelVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::publish_parser::variable_header::PublishVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::sub_ack::SubAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::subscribe::SubScribeVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::unsubscribe::UnSubScribeVariableHeader;

#[allow(dead_code)]
pub enum Packet {
    Connect {
        fixed: FixedHeader,
        variable: ConnectVariableHeader,
        payload: ConnectPayload,
    },
    ConnAck {
        fixed: FixedHeader,
        variable: ConnAckVariableHeader,
    },
    Publish {
        fixed: FixedHeader,
        variable: PublishVariableHeader,
        payload: PublishPayload,
    },
    PubAck {
        fixed: FixedHeader,
        variable: PubAckVariableHeader,
    },
    PubRec {
        fixed: FixedHeader,
        variable: PubRecVariableHeader,
    },
    PubRel {
        fixed: FixedHeader,
        variable: PubRelVariableHeader,
    },
    PubComp {
        fixed: FixedHeader,
        variable: PubCompVariableHeader,
    },
    Subscribe {
        fixed: FixedHeader,
        variable: SubScribeVariableHeader,
        payload: SubscribePayload,
    },
    SubAck {
        fixed: FixedHeader,
        variable: SubAckVariableHeader,
        payload: SubAckPayload,
    },
    Unsubscribe {
        fixed: FixedHeader,
        variable: UnSubScribeVariableHeader,
        payload: UnSubscribePayload,
    },
    UnsubAck {
        fixed: FixedHeader,
        variable: PubAckVariableHeader,
    },
    PingReq {
        fixed: FixedHeader,
    },
    PingResp {
        fixed: FixedHeader,
    },
    Disconnect {
        fixed: FixedHeader,
    },
}

#[allow(dead_code)]
impl Packet {
    fn read_fixed_header<T: MqttFixedHeaderCodec>(
        bytes: &mut impl ByteOperations,
    ) -> Result<T, MqttProtocolError> {
        T::decode(bytes)
    }
    fn read_variable_header<T: MqttVariableHeaderDecoder>(
        fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<T, MqttProtocolError> {
        T::decode(fixed_header, bytes)
    }

    fn read_payload<VariableHeader, T: MqttPayloadDecoder<VariableHeader>>(
        fixed_header: &FixedHeader,
        variable_header: &VariableHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<T, MqttProtocolError> {
        T::decode(fixed_header, variable_header, bytes)
    }
}

impl Decoder for Packet {
    fn decode(bytes: &mut impl ByteOperations) -> Result<Self, MqttProtocolError>
    where
        Self: Sized,
    {
        let fixed_header: FixedHeader = Self::read_fixed_header(bytes)?;

        match fixed_header.control_packet_type() {
            ControlPacketType::Connect => {
                let variable_header: ConnectVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;
                let payload: ConnectPayload =
                    Self::read_payload(&fixed_header, &variable_header, bytes)?;

                Ok(Packet::Connect {
                    fixed: fixed_header,
                    variable: variable_header,
                    payload,
                })
            }
            ControlPacketType::ConnAck => {
                let variable_header: ConnAckVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;

                Ok(Packet::ConnAck {
                    fixed: fixed_header,
                    variable: variable_header,
                })
            }
            ControlPacketType::Publish => {
                let variable_header: PublishVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;
                let payload: PublishPayload =
                    Self::read_payload(&fixed_header, &variable_header, bytes)?;

                Ok(Packet::Publish {
                    fixed: fixed_header,
                    variable: variable_header,
                    payload,
                })
            }
            ControlPacketType::PubAck => {
                let variable_header: PubAckVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;

                Ok(Packet::PubAck {
                    fixed: fixed_header,
                    variable: variable_header,
                })
            }
            ControlPacketType::PubRec => {
                let variable_header: PubRecVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;

                Ok(Packet::PubRec {
                    fixed: fixed_header,
                    variable: variable_header,
                })
            }
            ControlPacketType::PubRel => {
                let variable_header: PubRelVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;

                Ok(Packet::PubRel {
                    fixed: fixed_header,
                    variable: variable_header,
                })
            }
            ControlPacketType::PubComp => {
                let variable_header: PubCompVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;

                Ok(Packet::PubComp {
                    fixed: fixed_header,
                    variable: variable_header,
                })
            }
            ControlPacketType::Subscribe => {
                let variable_header: SubScribeVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;
                let payload: SubscribePayload =
                    Self::read_payload(&fixed_header, &variable_header, bytes)?;

                Ok(Packet::Subscribe {
                    fixed: fixed_header,
                    variable: variable_header,
                    payload,
                })
            }
            ControlPacketType::SubAck => {
                let variable_header: SubAckVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;
                let payload: SubAckPayload =
                    Self::read_payload(&fixed_header, &variable_header, bytes)?;

                Ok(Packet::SubAck {
                    fixed: fixed_header,
                    variable: variable_header,
                    payload,
                })
            }
            ControlPacketType::Unsubscribe => {
                let variable_header: UnSubScribeVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;
                let payload: UnSubscribePayload =
                    Self::read_payload(&fixed_header, &variable_header, bytes)?;

                Ok(Packet::Unsubscribe {
                    fixed: fixed_header,
                    variable: variable_header,
                    payload,
                })
            }
            ControlPacketType::UnsubAck => {
                let variable_header: PubAckVariableHeader =
                    Self::read_variable_header(&fixed_header, bytes)?;

                Ok(Packet::UnsubAck {
                    fixed: fixed_header,
                    variable: variable_header,
                })
            }
            ControlPacketType::PingReq => Ok(Packet::PingReq {
                fixed: fixed_header,
            }),
            ControlPacketType::PingResp => Ok(Packet::PingResp {
                fixed: fixed_header,
            }),
            ControlPacketType::Disconnect => Ok(Packet::Disconnect {
                fixed: fixed_header,
            }),
        }
    }
}

#[cfg(test)]
mod packet_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::codec::Decoder;
    use crate::protocol::common::protocol_level::ProtocolLevel;
    use crate::protocol::common::qos::QoSCode;
    use crate::protocol::mqtt4::control_packet_type::ControlPacketType;
    use crate::protocol::mqtt4::fixed_header_parser::fixed_header_flags::FixedHeaderFlags;
    use crate::protocol::mqtt4::packet::Packet;
    use crate::protocol::mqtt4::payload_parser::sub_ack_parser::payload::SubAckReturnCode;
    use crate::protocol::mqtt4::return_code::ReturnCode;
    use crate::utils::utf::utf_8_handler::write;
    use bytes::BytesMut;

    #[test]
    fn test_packet_decode_connect() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b0001_0000); // Connect packet type with reserved flags
        bytes.write_a_byte(12); // Remaining Length

        // Variable Header
        // Protocol Name
        write(&mut bytes, "MQTT").unwrap();
        // Protocol Level
        bytes.write_a_byte(4);
        // Connect Flags
        bytes.write_a_byte(0b0000_0010);
        // Keep Alive
        bytes.write_a_byte(0);
        bytes.write_a_byte(60);

        // Payload
        // Client Identifier
        write(&mut bytes, "client123").unwrap();

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::Connect {
            fixed,
            variable,
            payload,
        } = packet
        {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::Connect);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::Connect
            );
            assert_eq!(fixed.remaining_length(), 12);

            // Validate Variable Header
            assert_eq!(variable.protocol_level(), &ProtocolLevel::Mqtt3_1_1);
            let connect_flags = variable.connect_flags();
            assert!(!connect_flags.will_flag());
            assert!(connect_flags.clean_session());
            assert_eq!(variable.keep_alive(), 60);

            // Validate Payload
            assert_eq!(payload.client_id(), "client123");
        } else {
            panic!("Decoded packet is not of type Connect");
        }
    }

    #[test]
    fn test_packet_decode_conn_ack() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b0010_0000); // ConnAck packet type
        bytes.write_a_byte(2); // Remaining Length

        // Variable Header
        bytes.write_a_byte(0); // Acknowledge Flags
        bytes.write_a_byte(0); // Return Code (0 = Connection Accepted)
        let packet = Packet::decode(&mut bytes).unwrap();
        if let Packet::ConnAck { fixed, variable } = packet {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::ConnAck);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::ConnAck
            );
            assert_eq!(fixed.remaining_length(), 2);

            // Validate Variable Header
            assert!(!variable.session_present());
            assert_eq!(variable.return_code(), &ReturnCode::ConnectionAccepted);
        } else {
            panic!("Decoded packet is not of type ConnAck");
        }
    }

    #[test]
    fn test_packet_decode_publish() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b0011_0000); // Publish packet type with reserved flags
        bytes.write_a_byte(13); // Remaining Length

        // Variable Header
        // Topic Name
        write(&mut bytes, "test/topic").unwrap();

        // Payload
        write(&mut bytes, "Hello MQTT!").unwrap();

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::Publish {
            fixed,
            variable,
            payload,
        } = packet
        {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::Publish);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::Publish {
                    dup: false,
                    qos: QoSCode::Qos0,
                    retain: false
                }
            );
            assert_eq!(fixed.remaining_length(), 13);

            // Validate Variable Header
            assert_eq!(variable.topic_name(), "test/topic");
            assert_eq!(variable.packet_identifier(), None);

            // Validate Payload
            assert_eq!(payload.application_message(), "Hello MQTT!");
        } else {
            panic!("Decoded packet is not of type Publish");
        }
    }

    #[test]
    fn test_packet_decode_pub_ack() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b0100_0000); // PubAck packet type
        bytes.write_a_byte(2); // Remaining Length
        // Variable Header
        bytes.write_a_byte(0x12); // Packet Identifier MSB
        bytes.write_a_byte(0x34); // Packet Identifier LSB
        let packet = Packet::decode(&mut bytes).unwrap();
        if let Packet::PubAck { fixed, variable } = packet {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::PubAck);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::PubAck
            );
            assert_eq!(fixed.remaining_length(), 2);

            // Validate Variable Header
            assert_eq!(variable.packet_identifier(), 0x1234);
        } else {
            panic!("Decoded packet is not of type PubAck");
        }
    }

    #[test]
    fn test_packet_decode_pub_rec() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b0101_0000); // PubRec packet type
        bytes.write_a_byte(2); // Remaining Length
        // Variable Header
        bytes.write_a_byte(0x56); // Packet Identifier MSB
        bytes.write_a_byte(0x78); // Packet Identifier LSB
        let packet = Packet::decode(&mut bytes).unwrap();
        if let Packet::PubRec { fixed, variable } = packet {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::PubRec);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::PubRec
            );
            assert_eq!(fixed.remaining_length(), 2);

            // Validate Variable Header
            assert_eq!(variable.packet_identifier(), 0x5678);
        } else {
            panic!("Decoded packet is not of type PubRec");
        }
    }

    #[test]
    fn test_packet_decode_pub_rel() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b0110_0010); // PubRel packet type with reserved flags
        bytes.write_a_byte(2); // Remaining Length
        // Variable Header
        bytes.write_a_byte(0x9A); // Packet Identifier MSB
        bytes.write_a_byte(0xBC); // Packet Identifier LSB
        let packet = Packet::decode(&mut bytes).unwrap();
        if let Packet::PubRel { fixed, variable } = packet {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::PubRel);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::PubRel
            );
            assert_eq!(fixed.remaining_length(), 2);

            // Validate Variable Header
            assert_eq!(variable.packet_identifier(), 0x9ABC);
        } else {
            panic!("Decoded packet is not of type PubRel");
        }
    }

    #[test]
    fn test_packet_decode_pub_comp() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b0111_0000); // PubComp packet type
        bytes.write_a_byte(2); // Remaining Length
        // Variable Header
        bytes.write_a_byte(0xDE); // Packet Identifier MSB
        bytes.write_a_byte(0xF0); // Packet Identifier LSB
        let packet = Packet::decode(&mut bytes).unwrap();
        if let Packet::PubComp { fixed, variable } = packet {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::PubComp);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::PubComp
            );
            assert_eq!(fixed.remaining_length(), 2);

            // Validate Variable Header
            assert_eq!(variable.packet_identifier(), 0xDEF0);
        } else {
            panic!("Decoded packet is not of type PubComp");
        }
    }

    #[test]
    fn test_packet_decode_subscribe() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b1000_0010); // Subscribe packet type with reserved flags
        bytes.write_a_byte(9); // Remaining Length

        // Variable Header
        // Packet Identifier
        bytes.write_a_byte(0x00); // Packet Identifier MSB
        bytes.write_a_byte(0x0A); // Packet Identifier LSB

        // Payload
        // Topic Filter
        write(&mut bytes, "sensor/temp").unwrap();
        bytes.write_a_byte(1); // QoS
        write(&mut bytes, "sensor/temp1").unwrap();
        bytes.write_a_byte(2);

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::Subscribe {
            fixed,
            variable,
            payload,
        } = packet
        {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::Subscribe);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::Subscribe
            );
            assert_eq!(fixed.remaining_length(), 9);

            // Validate Variable Header
            assert_eq!(variable.packet_identifier(), 10);

            // Validate Payload
            let subscriptions = payload.subscription_and_qos_tuples();
            assert_eq!(subscriptions.len(), 2);
            assert_eq!(subscriptions[0].0, "sensor/temp");
            assert_eq!(subscriptions[0].1, QoSCode::Qos1);
            assert_eq!(subscriptions[0].0, "sensor/temp");
            assert_eq!(subscriptions[1].0, "sensor/temp1");
            assert_eq!(subscriptions[1].1, QoSCode::Qos2);
        } else {
            panic!("Decoded packet is not of type Subscribe");
        }
    }

    #[test]
    fn test_packet_decode_unsubscribe() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b1010_0010); // Unsubscribe packet type with reserved flags
        bytes.write_a_byte(9); // Remaining Length

        // Variable Header
        // Packet Identifier
        bytes.write_a_byte(0x00); // Packet Identifier MSB
        bytes.write_a_byte(0x0B); // Packet Identifier LSB

        // Payload
        // Topic Filter
        write(&mut bytes, "sensor/humidity").unwrap();
        write(&mut bytes, "sensor/pressure").unwrap();

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::Unsubscribe {
            fixed,
            variable,
            payload,
        } = packet
        {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::Unsubscribe);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::Unsubscribe
            );
            assert_eq!(fixed.remaining_length(), 9);

            // Validate Variable Header
            assert_eq!(variable.packet_identifier(), 11);

            // Validate Payload
            let topics = payload.topics();
            assert_eq!(topics.len(), 2);
            assert_eq!(topics[0], "sensor/humidity");
            assert_eq!(topics[0], "sensor/humidity");
            assert_eq!(topics[1], "sensor/pressure");
        } else {
            panic!("Decoded packet is not of type Unsubscribe");
        }
    }

    #[test]
    fn test_packet_decode_sub_ack() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b1001_0000); // SubAck packet type
        bytes.write_a_byte(5); // Remaining Length

        // Variable Header
        // Packet Identifier
        bytes.write_a_byte(0x00); // Packet Identifier MSB
        bytes.write_a_byte(0x0C); // Packet Identifier LSB

        // Payload
        bytes.write_a_byte(0); // Return Code QoS 0
        bytes.write_a_byte(1); // Return Code QoS 1
        bytes.write_a_byte(128); // Return Code Failure

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::SubAck {
            fixed,
            variable,
            payload,
        } = packet
        {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::SubAck);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::SubAck
            );
            assert_eq!(fixed.remaining_length(), 5);

            // Validate Variable Header
            assert_eq!(variable.packet_identifier(), 12);

            // Validate Payload
            let return_codes = payload.return_codes();
            assert_eq!(return_codes.len(), 3);
            assert_eq!(return_codes[0], SubAckReturnCode::Qos0);
            assert_eq!(return_codes[0], SubAckReturnCode::Qos0);
            assert_eq!(return_codes[1], SubAckReturnCode::Qos1);
            assert_eq!(return_codes[2], SubAckReturnCode::Failure);
        } else {
            panic!("Decoded packet is not of type SubAck");
        }
    }

    #[test]
    fn test_packet_decode_ping_req() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b1100_0000); // PingReq packet type
        bytes.write_a_byte(0); // Remaining Length

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::PingReq { fixed } = packet {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::PingReq);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::PingReq
            );
            assert_eq!(fixed.remaining_length(), 0);
        } else {
            panic!("Decoded packet is not of type PingReq");
        }
    }

    #[test]
    fn test_packet_decode_ping_resp() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b1101_0000); // PingResp packet type
        bytes.write_a_byte(0); // Remaining Length

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::PingResp { fixed } = packet {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::PingResp);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::PingResp
            );
            assert_eq!(fixed.remaining_length(), 0);
        } else {
            panic!("Decoded packet is not of type PingResp");
        }
    }

    #[test]
    fn test_packet_decode_disconnect() {
        let mut bytes = BytesMut::new();
        // Fixed Header
        bytes.write_a_byte(0b1110_0000); // Disconnect packet type
        bytes.write_a_byte(0); // Remaining Length

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::Disconnect { fixed } = packet {
            // Validate Fixed Header
            assert_eq!(fixed.control_packet_type(), &ControlPacketType::Disconnect);
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                &FixedHeaderFlags::Disconnect
            );
            assert_eq!(fixed.remaining_length(), 0);
        } else {
            panic!("Decoded packet is not of type Disconnect");
        }
    }
}
