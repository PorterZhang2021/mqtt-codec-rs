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
use crate::protocol::common::control_packet_type::ControlPacketType;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt4::fixed_header_parser::fixed_header_codec::MqttFixedHeaderCodec;
use crate::protocol::mqtt4::packet_parser::packet::Packet;
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
use crate::protocol::mqtt4::variable_header_parser::pub_comp_parser::variable_header::PubCompVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_rec_parser::variable_header::PubRecVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_rel_parser::variable_header::PubRelVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::publish_parser::variable_header::PublishVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::sub_ack_parser::variable_header::SubAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::subscribe_parser::variable_header::SubScribeVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::unsubscribe_parser::variable_header::UnSubScribeVariableHeader;

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

#[allow(dead_code)]
impl Packet {
    pub(in crate::protocol::mqtt4::packet_parser) fn read_fixed_header<T: MqttFixedHeaderCodec>(
        bytes: &mut impl ByteOperations,
    ) -> Result<T, MqttProtocolError> {
        T::decode(bytes)
    }
    pub(in crate::protocol::mqtt4::packet_parser) fn read_variable_header<
        T: MqttVariableHeaderDecoder,
    >(
        fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<T, MqttProtocolError> {
        T::decode(fixed_header, bytes)
    }

    pub(in crate::protocol::mqtt4::packet_parser) fn read_payload<
        VariableHeader,
        T: MqttPayloadDecoder<VariableHeader>,
    >(
        fixed_header: &FixedHeader,
        variable_header: &VariableHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<T, MqttProtocolError> {
        T::decode(fixed_header, variable_header, bytes)
    }
}
