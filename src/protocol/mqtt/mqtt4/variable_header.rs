use crate::protocol::byte_wrapper::byte_operations::ByteOperations;
use crate::protocol::mqtt::mqtt_protocol_error::MQTTProtocolError;
use crate::protocol::mqtt::mqtt4::control_packet_type::ControlPacketType;
use crate::protocol::mqtt::mqtt4::packet::conn_ack::ConnAckVariableHeader;
use crate::protocol::mqtt::mqtt4::packet::connect::ConnectVariableHeader;
use crate::protocol::mqtt::mqtt4::packet::pub_ack::PubAckVariableHeader;
use crate::protocol::mqtt::mqtt4::packet::publish::PublishVariableHeader;
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

impl VariableHeader {
    // todo this method needs to be expanded to support all packet types
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
