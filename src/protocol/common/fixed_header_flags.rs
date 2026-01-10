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

use crate::protocol::common::control_packet_type::ControlPacketType;
use crate::protocol::common::qos::QoSCode;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::utils::radix::radix_handler;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FixedHeaderFlags {
    Publish {
        dup: bool,
        qos: QoSCode,
        retain: bool,
    },
    Connect,
    ConnAck,
    PubAck,
    PubRec,
    PubRel,
    PubComp,
    Subscribe,
    SubAck,
    Unsubscribe,
    UnsubAck,
    PingReq,
    PingResp,
    Disconnect,
    Auth,
}

#[allow(dead_code)]
impl FixedHeaderFlags {
    pub(crate) fn parse(
        control_packet_type: &ControlPacketType,
        binary_byte: u8,
    ) -> Result<Self, MqttProtocolError> {
        Self::verify(control_packet_type.clone(), binary_byte)?;
        Self::create_factory(control_packet_type, binary_byte)
    }

    pub(crate) fn encode(&self) -> u8 {
        match self {
            FixedHeaderFlags::Publish { dup, qos, retain } => {
                let mut flags: u8 = ControlPacketType::Publish.as_u8();
                if *dup {
                    flags |= 0b0000_1000;
                }
                flags |= match qos {
                    QoSCode::Qos0 => 0b0000_0000,
                    QoSCode::Qos1 => 0b0000_0010,
                    QoSCode::Qos2 => 0b0000_0100,
                };
                if *retain {
                    flags |= 0b0000_0001;
                }
                flags
            }
            FixedHeaderFlags::Connect => {
                let mut flags = ControlPacketType::Connect.as_u8();
                flags &= 0b1111_0000;
                flags
            }
            FixedHeaderFlags::ConnAck => {
                let mut flags = ControlPacketType::ConnAck.as_u8();
                flags &= 0b1111_0000;
                flags
            }
            FixedHeaderFlags::PubAck => {
                let mut flags = ControlPacketType::PubAck.as_u8();
                flags &= 0b1111_0000;
                flags
            }
            FixedHeaderFlags::PubRec => {
                let mut flags = ControlPacketType::PubRec.as_u8();
                flags &= 0b1111_0000;
                flags
            }
            FixedHeaderFlags::PubRel => {
                let mut flags = ControlPacketType::PubRel.as_u8();
                flags |= 0b0000_0010;
                flags
            }
            FixedHeaderFlags::PubComp => {
                let mut flags = ControlPacketType::PubComp.as_u8();
                flags &= 0b1111_0000;
                flags
            }
            FixedHeaderFlags::Subscribe => {
                let mut flags = ControlPacketType::Subscribe.as_u8();
                flags |= 0b0000_0010;
                flags
            }
            FixedHeaderFlags::SubAck => {
                let mut flags = ControlPacketType::SubAck.as_u8();
                flags &= 0b1111_0000;
                flags
            }
            FixedHeaderFlags::Unsubscribe => {
                let mut flags = ControlPacketType::Unsubscribe.as_u8();
                flags |= 0b0000_0010;
                flags
            }
            FixedHeaderFlags::UnsubAck => {
                let mut flags = ControlPacketType::UnsubAck.as_u8();
                flags &= 0b1111_0000;
                flags
            }
            FixedHeaderFlags::PingReq => {
                let mut flags = ControlPacketType::PingReq.as_u8();
                flags &= 0b1111_0000;
                flags
            }
            FixedHeaderFlags::PingResp => {
                let mut flags = ControlPacketType::PingResp.as_u8();
                flags &= 0b1111_0000;
                flags
            }
            FixedHeaderFlags::Disconnect => {
                let mut flags = ControlPacketType::Disconnect.as_u8();
                flags &= 0b1111_0000;
                flags
            }
            FixedHeaderFlags::Auth => {
                let mut flags = ControlPacketType::Auth.as_u8();
                flags &= 0b1111_0000;
                flags
            }
        }
    }

    pub(crate) fn verify(
        control_packet_type: ControlPacketType,
        binary_byte: u8,
    ) -> Result<(), MqttProtocolError> {
        match control_packet_type {
            ControlPacketType::Connect
            | ControlPacketType::ConnAck
            | ControlPacketType::PubAck
            | ControlPacketType::PubRec
            | ControlPacketType::PubComp
            | ControlPacketType::UnsubAck
            | ControlPacketType::PingReq
            | ControlPacketType::PingResp
            | ControlPacketType::Disconnect
            | ControlPacketType::SubAck => {
                Ok(Self::check_reserved_value(binary_byte, 0b0000_0000)?)
            }
            ControlPacketType::PubRel
            | ControlPacketType::Subscribe
            | ControlPacketType::Unsubscribe => {
                Ok(Self::check_reserved_value(binary_byte, 0b0000_0010)?)
            }
            ControlPacketType::Publish => Ok(()),
            ControlPacketType::Auth => Ok(()),
        }
    }

    pub(self) fn check_reserved_value(
        binary_byte: u8,
        reserved_value: u8,
    ) -> Result<(), MqttProtocolError> {
        if radix_handler::low_nibble(binary_byte) != reserved_value {
            return Err(MqttProtocolError::InvalidFixedHeaderFlags);
        }
        Ok(())
    }

    pub(self) fn create_factory(
        control_packet_type: &ControlPacketType,
        binary_byte: u8,
    ) -> Result<Self, MqttProtocolError> {
        match control_packet_type {
            ControlPacketType::Publish => Self::create_publish_fixed_header_flags(binary_byte),
            ControlPacketType::Connect => Ok(FixedHeaderFlags::Connect),
            ControlPacketType::ConnAck => Ok(FixedHeaderFlags::ConnAck),
            ControlPacketType::PubAck => Ok(FixedHeaderFlags::PubAck),
            ControlPacketType::PubRec => Ok(FixedHeaderFlags::PubRec),
            ControlPacketType::PubRel => Ok(FixedHeaderFlags::PubRel),
            ControlPacketType::PubComp => Ok(FixedHeaderFlags::PubComp),
            ControlPacketType::Subscribe => Ok(FixedHeaderFlags::Subscribe),
            ControlPacketType::SubAck => Ok(FixedHeaderFlags::SubAck),
            ControlPacketType::Unsubscribe => Ok(FixedHeaderFlags::Unsubscribe),
            ControlPacketType::UnsubAck => Ok(FixedHeaderFlags::UnsubAck),
            ControlPacketType::PingReq => Ok(FixedHeaderFlags::PingReq),
            ControlPacketType::PingResp => Ok(FixedHeaderFlags::PingResp),
            ControlPacketType::Disconnect => Ok(FixedHeaderFlags::Disconnect),
            ControlPacketType::Auth => Ok(FixedHeaderFlags::Auth),
        }
    }

    pub(self) fn create_publish_fixed_header_flags(
        binary_byte: u8,
    ) -> Result<Self, MqttProtocolError> {
        let low4bits = radix_handler::low_nibble(binary_byte);
        let dup = (low4bits & 0b0000_1000) >> 3 == 1;
        let qos = (low4bits & 0b0000_0110) >> 1;
        let qos_code = QoSCode::try_from(qos)?;
        let retain = (low4bits & 0b0000_0001) == 1;
        Ok(FixedHeaderFlags::Publish {
            dup,
            qos: qos_code,
            retain,
        })
    }
}

#[cfg(test)]
mod fixed_header_flags_tests {
    use crate::protocol::common::control_packet_type::ControlPacketType;
    use crate::protocol::common::fixed_header_flags::FixedHeaderFlags;
    use crate::protocol::common::qos::QoSCode;
    use crate::protocol::mqtt_protocol_error::MqttProtocolError;

    #[test]
    fn fixed_header_connect_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::Connect.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Connect);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok())
    }
    #[test]
    fn fixed_header_connack_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::ConnAck.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::ConnAck);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_publish_reserved_flags_should_be_0000_to_1111() {
        let byte = FixedHeaderFlags::Publish {
            dup: false,
            qos: QoSCode::Qos0,
            retain: false,
        }
        .encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Publish);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_puback_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::PubAck.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::PubAck);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_pubrec_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::PubRec.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::PubRec);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_pubrel_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::PubRel.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::PubRel);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_pubcomp_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::PubComp.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::PubComp);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_subscribe_reserved_flags_should_be_0010() {
        let byte = FixedHeaderFlags::Subscribe.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Subscribe);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_suback_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::SubAck.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::SubAck);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_unsubscribe_reserved_flags_should_be_0010() {
        let byte = FixedHeaderFlags::Unsubscribe.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Unsubscribe);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_unsuback_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::UnsubAck.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::UnsubAck);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_pingreq_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::PingReq.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::PingReq);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_pingresp_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::PingResp.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::PingResp);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }
    #[test]
    fn fixed_header_disconnect_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::Disconnect.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Disconnect);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }

    #[test]
    fn fixed_header_auth_reserved_flags_should_be_0000() {
        let byte = FixedHeaderFlags::Auth.encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Auth);
        assert!(FixedHeaderFlags::verify(packet_type, byte).is_ok());
    }

    #[test]
    fn fixed_header_invalid_reserved_flags_should_error() {
        let byte = 0b1110_0010;
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Disconnect);
        let result = FixedHeaderFlags::verify(packet_type, byte);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(MqttProtocolError::InvalidFixedHeaderFlags)
        ))
    }

    #[test]
    fn fixed_header_publish_extract_reserved_flags_should_get_dup_value() {
        let byte = FixedHeaderFlags::Publish {
            dup: true,
            qos: QoSCode::Qos0,
            retain: false,
        }
        .encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Publish);
        let publish_flags = FixedHeaderFlags::parse(&packet_type, byte).unwrap();
        if let FixedHeaderFlags::Publish {
            dup,
            qos: _,
            retain: _,
        } = publish_flags
        {
            assert!(dup);
        }
    }

    #[test]
    fn fixed_header_publish_extract_reserved_flags_should_get_retain_value() {
        let byte = FixedHeaderFlags::Publish {
            dup: false,
            qos: QoSCode::Qos0,
            retain: true,
        }
        .encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Publish);
        let publish_flags = FixedHeaderFlags::parse(&packet_type, byte).unwrap();
        if let FixedHeaderFlags::Publish {
            dup: _,
            qos: _,
            retain,
        } = publish_flags
        {
            assert!(retain);
        }
    }
    #[test]
    fn fixed_header_publish_extract_reserved_flags_should_get_qos_0_value() {
        let byte = FixedHeaderFlags::Publish {
            dup: false,
            qos: QoSCode::Qos0,
            retain: false,
        }
        .encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Publish);
        let publish_flags = FixedHeaderFlags::parse(&packet_type, byte).unwrap();
        if let FixedHeaderFlags::Publish {
            dup: _,
            qos,
            retain: _,
        } = publish_flags
        {
            assert_eq!(qos, QoSCode::Qos0);
        }
    }
    #[test]
    fn fixed_header_publish_extract_reserved_flags_should_get_qos_value() {
        let byte = FixedHeaderFlags::Publish {
            dup: false,
            qos: QoSCode::Qos2,
            retain: false,
        }
        .encode();
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Publish);
        let publish_flags = FixedHeaderFlags::parse(&packet_type, byte).unwrap();
        if let FixedHeaderFlags::Publish {
            dup: _,
            qos,
            retain: _,
        } = publish_flags
        {
            assert_eq!(qos, QoSCode::Qos2);
        }
    }
    #[test]
    fn fixed_header_publish_extract_reserved_flags_qos_invalid_should_error() {
        let byte = 0b0011_0110;
        let packet_type = ControlPacketType::parse(byte).unwrap();
        assert_eq!(packet_type, ControlPacketType::Publish);
        let result = FixedHeaderFlags::parse(&packet_type, byte);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(MqttProtocolError::QoSLevelNotSupported(3))
        ))
    }
}
