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
use crate::protocol::mqtt4::variable_header_parser::conn_ack::ConnAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::connect::ConnectVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::pub_ack::PubAckVariableHeader;
use crate::protocol::mqtt4::variable_header_parser::publish::PublishVariableHeader;

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
}

#[allow(dead_code)]
impl VariableHeader {
    // todo this method needs to be expanded to support all variable_header_parser types
    fn parse(
        bytes: &mut impl ByteOperations,
        packet_type: &ControlPacketType,
    ) -> Result<VariableHeader, MQTTProtocolError> {
        match packet_type {
            ControlPacketType::Connect => Ok(VariableHeader::Connect {
                connect_variable_header: ConnectVariableHeader::parse(bytes)?,
            }),
            ControlPacketType::ConnAck => Ok(VariableHeader::ConnAck {
                conn_ack_variable_header: ConnAckVariableHeader::parse(bytes)?,
            }),
            _ => Err(MQTTProtocolError::UnsupportedPacketType),
        }
    }
}

impl VariableHeader {}

mod variable_header_tests {
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
