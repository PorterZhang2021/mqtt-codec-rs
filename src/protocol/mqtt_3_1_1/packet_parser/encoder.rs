use crate::protocol::codec::Encoder;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::protocol::mqtt_3_1_1::fixed_header_parser::fixed_header_codec::FixedHeaderEncoder;
use crate::protocol::mqtt_3_1_1::packet_parser::packet::Packet;
use crate::protocol::mqtt_3_1_1::payload_parser::payload_codec::PayloadEncoder;
use crate::protocol::mqtt_3_1_1::payload_parser::none_parser::encoder::NonePayload;
use crate::protocol::mqtt_3_1_1::variable_header_parser::variable_header_codec::VariableHeaderEncoder;
use crate::protocol::mqtt_3_1_1::variable_header_parser::none_variable_header_parser::variable_header::NoneVariableHeader;
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

#[allow(dead_code)]
impl Encoder for Packet {
    fn encode(&mut self) -> Result<Vec<u8>, MqttProtocolError>
    where
        Self: Sized,
    {
        match self {
            Packet::Connect {
                fixed,
                variable,
                payload,
            } => Self::encode(fixed, variable, payload),
            Packet::Publish {
                fixed,
                variable,
                payload,
            } => Self::encode(fixed, variable, payload),
            Packet::Subscribe {
                fixed,
                variable,
                payload,
            } => Self::encode(fixed, variable, payload),
            Packet::SubAck {
                fixed,
                variable,
                payload,
            } => Self::encode(fixed, variable, payload),
            Packet::Unsubscribe {
                fixed,
                variable,
                payload,
            } => Self::encode(fixed, variable, payload),
            Packet::ConnAck { fixed, variable } => Self::encode(fixed, variable, &NonePayload),
            Packet::PubAck { fixed, variable } => Self::encode(fixed, variable, &NonePayload),
            Packet::PubRec { fixed, variable } => Self::encode(fixed, variable, &NonePayload),
            Packet::PubRel { fixed, variable } => Self::encode(fixed, variable, &NonePayload),
            Packet::PubComp { fixed, variable } => Self::encode(fixed, variable, &NonePayload),
            Packet::UnsubAck { fixed, variable } => Self::encode(fixed, variable, &NonePayload),
            Packet::PingReq { fixed }
            | Packet::PingResp { fixed }
            | Packet::Disconnect { fixed } => {
                Self::encode(fixed, &NoneVariableHeader, &NonePayload)
            }
        }
    }
}

#[allow(dead_code)]
impl Packet {
    pub(crate) fn encode_payload<T: PayloadEncoder>(
        payload: &T,
    ) -> Result<Vec<u8>, MqttProtocolError> {
        payload.encode()
    }

    pub(crate) fn encode_variable_header<T: VariableHeaderEncoder>(
        variable_header: &T,
    ) -> Result<Vec<u8>, MqttProtocolError> {
        variable_header.encode()
    }

    pub(crate) fn encode_fixed_header<T: FixedHeaderEncoder>(
        fixed_header: &mut T,
        remaining_length: u32,
    ) -> Result<Vec<u8>, MqttProtocolError> {
        fixed_header.encode(remaining_length)
    }

    fn encode<F: FixedHeaderEncoder, V: VariableHeaderEncoder, P: PayloadEncoder>(
        fixed: &mut F,
        variable: &V,
        payload: &P,
    ) -> Result<Vec<u8>, MqttProtocolError> {
        let variable_bytes = Self::encode_variable_header(variable)?;
        let payload_bytes = Self::encode_payload(payload)?;

        let remaining_length = (variable_bytes.len() + payload_bytes.len()) as u32;
        let encode_fixed = Self::encode_fixed_header(fixed, remaining_length)?;

        let mut bytes =
            Vec::with_capacity(encode_fixed.len() + variable_bytes.len() + payload_bytes.len());
        bytes.extend(encode_fixed);
        bytes.extend(variable_bytes);
        bytes.extend(payload_bytes);

        Ok(bytes)
    }
}
