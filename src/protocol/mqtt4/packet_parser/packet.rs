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

use crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt4::payload_parser::connect_parser::payload::ConnectPayload;
use crate::protocol::mqtt4::payload_parser::publish_parser::payload::PublishPayload;
use crate::protocol::mqtt4::payload_parser::sub_ack_parser::payload::SubAckPayload;
use crate::protocol::mqtt4::payload_parser::subscribe_parser::payload::SubscribePayload;
use crate::protocol::mqtt4::payload_parser::unsubscribe_parser::payload::UnSubscribePayload;
use crate::protocol::mqtt4::variable_header_parser::conn_ack_parser::variable_header::ConnAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::connect_parser::variable_header::ConnectVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_ack_parser::variable_header::PubAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_comp_parser::variable_header::PubCompVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_rec_parser::variable_header::PubRecVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_rel_parser::variable_header::PubRelVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::publish_parser::variable_header::PublishVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::sub_ack_parser::variable_header::SubAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::subscribe_parser::variable_header::SubscribeVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::unsubscribe_parser::variable_header::UnSubscribeVariableHeader;
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
        variable: SubscribeVariableHeader,
        payload: SubscribePayload,
    },
    SubAck {
        fixed: FixedHeader,
        variable: SubAckVariableHeader,
        payload: SubAckPayload,
    },
    Unsubscribe {
        fixed: FixedHeader,
        variable: UnSubscribeVariableHeader,
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

#[cfg(test)]
mod packet_tests {
    use crate::protocol::codec::{Decoder, Encoder};
    use crate::protocol::common::control_packet_type::ControlPacketType;
    use crate::protocol::common::protocol_level::ProtocolLevel;
    use crate::protocol::common::qos::QoSCode;
    use crate::protocol::common::return_code::ReturnCode;
    use crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader;
    use crate::protocol::mqtt4::fixed_header_parser::fixed_header_flags::FixedHeaderFlags;
    use crate::protocol::mqtt4::packet_parser::packet::Packet;
    use crate::protocol::mqtt4::payload_parser::connect_parser::payload::ConnectPayload;
    use crate::protocol::mqtt4::payload_parser::publish_parser::payload::PublishPayload;
    use crate::protocol::mqtt4::payload_parser::sub_ack_parser::payload::SubAckPayload;
    use crate::protocol::mqtt4::payload_parser::sub_ack_parser::payload::SubAckReturnCode;
    use crate::protocol::mqtt4::payload_parser::subscribe_parser::payload::SubscribePayload;
    use crate::protocol::mqtt4::payload_parser::unsubscribe_parser::payload::UnSubscribePayload;
    use crate::protocol::mqtt4::variable_header_parser::conn_ack_parser::variable_header::ConnAckVariableHeader;
    use crate::protocol::mqtt4::variable_header_parser::connect_parser::variable_header::{
        ConnectFlags, ConnectVariableHeader,
    };
    use crate::protocol::mqtt4::variable_header_parser::pub_ack_parser::variable_header::PubAckVariableHeader;
    use crate::protocol::mqtt4::variable_header_parser::pub_comp_parser::variable_header::PubCompVariableHeader;
    use crate::protocol::mqtt4::variable_header_parser::pub_rec_parser::variable_header::PubRecVariableHeader;
    use crate::protocol::mqtt4::variable_header_parser::pub_rel_parser::variable_header::PubRelVariableHeader;
    use crate::protocol::mqtt4::variable_header_parser::publish_parser::variable_header::PublishVariableHeader;
    use crate::protocol::mqtt4::variable_header_parser::sub_ack_parser::variable_header::SubAckVariableHeader;
    use crate::protocol::mqtt4::variable_header_parser::subscribe_parser::variable_header::SubscribeVariableHeader;
    use crate::protocol::mqtt4::variable_header_parser::unsubscribe_parser::variable_header::UnSubscribeVariableHeader;
    use bytes::BytesMut;

    #[test]
    fn test_packet_decode_connect() {
        let mut bytes = BytesMut::new();

        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::Connect, FixedHeaderFlags::Connect);
        let expect_connect_flags =
            ConnectFlags::new(false, false, false, QoSCode::Qos0, false, true).unwrap();
        let expect_connect_variable_header =
            ConnectVariableHeader::new(ProtocolLevel::Mqtt3_1_1, expect_connect_flags, 60);
        let expect_payload = ConnectPayload::new("client123".to_string(), None, None, None, None);

        let mut expect_connect_packet = Packet::Connect {
            fixed: expect_fixed_header.clone(),
            variable: expect_connect_variable_header.clone(),
            payload: expect_payload.clone(),
        };

        let encode_expect_connect_packet = expect_connect_packet.encode().unwrap();

        bytes.extend_from_slice(&encode_expect_connect_packet);

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::Connect {
            fixed,
            variable,
            payload,
        } = packet
        {
            // Validate Fixed Header
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::Connect { fixed, .. } = &expect_connect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );

            // Validate Variable Header
            assert_eq!(
                variable.protocol_level(),
                expect_connect_variable_header.protocol_level()
            );
            assert_eq!(
                variable.connect_flags(),
                expect_connect_variable_header.connect_flags()
            );
            assert_eq!(
                variable.keep_alive(),
                expect_connect_variable_header.keep_alive()
            );

            // Validate Payload
            assert_eq!(payload.client_id(), expect_payload.client_id());
        } else {
            panic!("Decoded packet is not of type Connect");
        }
    }

    #[test]
    fn test_packet_decode_conn_ack() {
        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::ConnAck, FixedHeaderFlags::ConnAck);
        let expect_variable_header =
            ConnAckVariableHeader::new(false, ReturnCode::ConnectionAccepted);
        let mut expect_packet = Packet::ConnAck {
            fixed: expect_fixed_header.clone(),
            variable: expect_variable_header.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();
        if let Packet::ConnAck { fixed, variable } = packet {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::ConnAck { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );
            assert_eq!(
                variable.session_present(),
                expect_variable_header.session_present()
            );
            assert_eq!(variable.return_code(), expect_variable_header.return_code());
        } else {
            panic!("Decoded packet is not of type ConnAck");
        }
    }

    #[test]
    fn test_packet_decode_publish() {
        let expect_fixed_header = FixedHeader::new(
            ControlPacketType::Publish,
            FixedHeaderFlags::Publish {
                dup: false,
                qos: QoSCode::Qos0,
                retain: false,
            },
        );
        let expect_variable_header = PublishVariableHeader::new("test/topic".to_string(), None);
        let expect_payload = PublishPayload::new("Hello MQTT!".to_string());

        let mut expect_packet = Packet::Publish {
            fixed: expect_fixed_header.clone(),
            variable: expect_variable_header.clone(),
            payload: expect_payload.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::Publish {
            fixed,
            variable,
            payload,
        } = packet
        {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::Publish { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );

            assert_eq!(variable.topic_name(), expect_variable_header.topic_name());
            assert_eq!(
                variable.packet_identifier(),
                expect_variable_header.packet_identifier()
            );
            assert_eq!(
                payload.application_message(),
                expect_payload.application_message()
            );
        } else {
            panic!("Decoded packet is not of type Publish");
        }
    }

    #[test]
    fn test_packet_decode_pub_ack() {
        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::PubAck, FixedHeaderFlags::PubAck);
        let expect_variable_header = PubAckVariableHeader::new(0x1234);
        let mut expect_packet = Packet::PubAck {
            fixed: expect_fixed_header.clone(),
            variable: expect_variable_header.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();
        if let Packet::PubAck { fixed, variable } = packet {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::PubAck { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );
            assert_eq!(
                variable.packet_identifier(),
                expect_variable_header.packet_identifier()
            );
        } else {
            panic!("Decoded packet is not of type PubAck");
        }
    }

    #[test]
    fn test_packet_decode_pub_rec() {
        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::PubRec, FixedHeaderFlags::PubRec);
        let expect_variable_header = PubRecVariableHeader::new(0x5678);
        let mut expect_packet = Packet::PubRec {
            fixed: expect_fixed_header.clone(),
            variable: expect_variable_header.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();
        if let Packet::PubRec { fixed, variable } = packet {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::PubRec { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );
            assert_eq!(
                variable.packet_identifier(),
                expect_variable_header.packet_identifier()
            );
        } else {
            panic!("Decoded packet is not of type PubRec");
        }
    }

    #[test]
    fn test_packet_decode_pub_rel() {
        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::PubRel, FixedHeaderFlags::PubRel);
        let expect_variable_header = PubRelVariableHeader::new(0x9ABC);
        let mut expect_packet = Packet::PubRel {
            fixed: expect_fixed_header.clone(),
            variable: expect_variable_header.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();
        if let Packet::PubRel { fixed, variable } = packet {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::PubRel { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );
            assert_eq!(
                variable.packet_identifier(),
                expect_variable_header.packet_identifier()
            );
        } else {
            panic!("Decoded packet is not of type PubRel");
        }
    }

    #[test]
    fn test_packet_decode_pub_comp() {
        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::PubComp, FixedHeaderFlags::PubComp);
        let expect_variable_header = PubCompVariableHeader::new(0xDEF0);
        let mut expect_packet = Packet::PubComp {
            fixed: expect_fixed_header.clone(),
            variable: expect_variable_header.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();
        if let Packet::PubComp { fixed, variable } = packet {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::PubComp { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );
            assert_eq!(
                variable.packet_identifier(),
                expect_variable_header.packet_identifier()
            );
        } else {
            panic!("Decoded packet is not of type PubComp");
        }
    }

    #[test]
    fn test_packet_decode_subscribe() {
        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::Subscribe, FixedHeaderFlags::Subscribe);
        let expect_variable_header = SubscribeVariableHeader::new(10);
        let expect_payload = SubscribePayload::new(vec![
            ("sensor/temp".to_string(), QoSCode::Qos1),
            ("sensor/temp1".to_string(), QoSCode::Qos2),
        ]);

        let mut expect_packet = Packet::Subscribe {
            fixed: expect_fixed_header.clone(),
            variable: expect_variable_header.clone(),
            payload: expect_payload.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::Subscribe {
            fixed,
            variable,
            payload,
        } = packet
        {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::Subscribe { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );

            assert_eq!(
                variable.packet_identifier(),
                expect_variable_header.packet_identifier()
            );
            let subscriptions = payload.subscription_and_qos_tuples();
            assert_eq!(
                subscriptions.len(),
                expect_payload.subscription_and_qos_tuples().len()
            );
            assert_eq!(
                subscriptions[0].0,
                expect_payload.subscription_and_qos_tuples()[0].0
            );
            assert_eq!(
                subscriptions[0].1,
                expect_payload.subscription_and_qos_tuples()[0].1
            );
            assert_eq!(
                subscriptions[1].0,
                expect_payload.subscription_and_qos_tuples()[1].0
            );
            assert_eq!(
                subscriptions[1].1,
                expect_payload.subscription_and_qos_tuples()[1].1
            );
        } else {
            panic!("Decoded packet is not of type Subscribe");
        }
    }

    #[test]
    fn test_packet_decode_unsubscribe() {
        let expect_fixed_header = FixedHeader::new(
            ControlPacketType::Unsubscribe,
            FixedHeaderFlags::Unsubscribe,
        );
        let expect_variable_header = UnSubscribeVariableHeader::new(11);
        let expect_payload = UnSubscribePayload::new(vec![
            "sensor/humidity".to_string(),
            "sensor/pressure".to_string(),
        ]);

        let mut expect_packet = Packet::Unsubscribe {
            fixed: expect_fixed_header.clone(),
            variable: expect_variable_header.clone(),
            payload: expect_payload.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::Unsubscribe {
            fixed,
            variable,
            payload,
        } = packet
        {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::Unsubscribe { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );

            assert_eq!(
                variable.packet_identifier(),
                expect_variable_header.packet_identifier()
            );
            let topics = payload.topics();
            assert_eq!(topics.len(), expect_payload.topics().len());
            assert_eq!(topics[0], expect_payload.topics()[0]);
            assert_eq!(topics[1], expect_payload.topics()[1]);
        } else {
            panic!("Decoded packet is not of type Unsubscribe");
        }
    }

    #[test]
    fn test_packet_decode_sub_ack() {
        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::SubAck, FixedHeaderFlags::SubAck);
        let expect_variable_header = SubAckVariableHeader::new(12);
        let expect_payload = SubAckPayload::new(vec![
            SubAckReturnCode::Qos0,
            SubAckReturnCode::Qos1,
            SubAckReturnCode::Failure,
        ]);

        let mut expect_packet = Packet::SubAck {
            fixed: expect_fixed_header.clone(),
            variable: expect_variable_header.clone(),
            payload: expect_payload.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::SubAck {
            fixed,
            variable,
            payload,
        } = packet
        {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::SubAck { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );

            assert_eq!(
                variable.packet_identifier(),
                expect_variable_header.packet_identifier()
            );
            let return_codes = payload.return_codes();
            assert_eq!(return_codes.len(), expect_payload.return_codes().len());
            assert_eq!(return_codes[0], expect_payload.return_codes()[0]);
            assert_eq!(return_codes[1], expect_payload.return_codes()[1]);
            assert_eq!(return_codes[2], expect_payload.return_codes()[2]);
        } else {
            panic!("Decoded packet is not of type SubAck");
        }
    }

    #[test]
    fn test_packet_decode_ping_req() {
        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::PingReq, FixedHeaderFlags::PingReq);
        let mut expect_packet = Packet::PingReq {
            fixed: expect_fixed_header.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::PingReq { fixed } = packet {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::PingReq { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );
        } else {
            panic!("Decoded packet is not of type PingReq");
        }
    }

    #[test]
    fn test_packet_decode_ping_resp() {
        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::PingResp, FixedHeaderFlags::PingResp);
        let mut expect_packet = Packet::PingResp {
            fixed: expect_fixed_header.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::PingResp { fixed } = packet {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::PingResp { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );
        } else {
            panic!("Decoded packet is not of type PingResp");
        }
    }

    #[test]
    fn test_packet_decode_disconnect() {
        let expect_fixed_header =
            FixedHeader::new(ControlPacketType::Disconnect, FixedHeaderFlags::Disconnect);
        let mut expect_packet = Packet::Disconnect {
            fixed: expect_fixed_header.clone(),
        };

        let encoded = expect_packet.encode().unwrap();
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&encoded);

        let packet = Packet::decode(&mut bytes).unwrap();

        if let Packet::Disconnect { fixed } = packet {
            assert_eq!(
                fixed.control_packet_type(),
                expect_fixed_header.control_packet_type()
            );
            assert_eq!(
                fixed.fixed_header_reserved_flags(),
                expect_fixed_header.fixed_header_reserved_flags()
            );
            assert_eq!(
                fixed.remaining_length(),
                if let Packet::Disconnect { fixed, .. } = &expect_packet {
                    fixed.remaining_length()
                } else {
                    0
                }
            );
        } else {
            panic!("Decoded packet is not of type Disconnect");
        }
    }
}
