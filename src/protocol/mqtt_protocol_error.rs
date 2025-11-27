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

use crate::utils::code_error::CodeError;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum MQTTProtocolError {
    #[error("Malformed variable_header_parser")]
    MalformedPacket,

    #[error("Invalid variable_header_parser type: reserved bits are forbidden to use")]
    InvalidPacketType,

    #[error("This Control Packet type reserved flag is invalid")]
    InvalidFixedHeaderFlags,

    #[error("QoS can support 0, 1, 2, the specified QoS {0} level is not supported")]
    QoSLevelNotSupported(u8),

    #[error("Remaining Length field is malformed")]
    MalformedRemainingLength,

    #[error("Packet does not have enough bytes")]
    PacketTooShort,

    #[error("Protocol Name error: {0}")]
    ProtocolNameError(String),

    #[error("Protocol Level no support: {0}")]
    ProtocolLevelNoSupport(u8),

    #[error("from CodeError: {0}")]
    CodeError(#[from] CodeError),

    #[error("Invalid Will QoS level: {0}")]
    InvalidWillQoS(u8),

    #[error("Unsupported Packet Type")]
    UnsupportedPacketType,

    #[error("Return Code is reserved and cannot be used")]
    ReservedReturnCode,

    #[error("Client Identifier is invalid")]
    InvalidClientId,
}
