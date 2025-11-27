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

use crate::protocol::mqtt4::fixed_header::FixedHeader;
use crate::protocol::mqtt4::payload_parser::connect::ConnectPayload;
use crate::protocol::mqtt4::variable_header::VariableHeader;

#[allow(dead_code)]
struct ConnectPacket {
    fixed_header: FixedHeader,
    variable_header: VariableHeader,
    payload: ConnectPayload,
}
/*
impl Codec<ConnectPacket> for ConnectPacket {
    fn decode(
        bytes: &mut impl crate::byte_adapter::byte_operations::ByteOperations,
    ) -> Result<ConnectPacket, crate::protocol::mqtt_protocol_error::MQTTProtocolError> {
        let fixed_header = FixedHeader::parse(bytes)?;
        let variable_header = VariableHeader::parse_connect_variable_header(bytes)?;
        let payload = ConnectPayload::parse(bytes)?;

        Ok(ConnectPacket {
            fixed_header,
            variable_header,
            payload,
        })
    }

    fn encode(
        _packet: ConnectPacket,
    ) -> Result<&'static [u8], crate::protocol::mqtt_protocol_error::MQTTProtocolError> {
        unimplemented!()
    }
}*/
