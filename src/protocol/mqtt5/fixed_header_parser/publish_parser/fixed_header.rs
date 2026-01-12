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

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PublishFixedHeader {
    control_packet_type: ControlPacketType,
    fixed_header_reserved_flags: PublishFixedHeaderReservedFlags,
    remaining_len: u32,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PublishFixedHeaderReservedFlags {
    dup: bool,
    qos: QoSCode,
    retain: bool,
}
#[allow(dead_code)]
impl PublishFixedHeaderReservedFlags {
    pub(crate) fn new(dup: bool, qos: QoSCode, retain: bool) -> Self {
        PublishFixedHeaderReservedFlags { dup, qos, retain }
    }

    pub(crate) fn dup(&self) -> bool {
        self.dup
    }

    pub(crate) fn qos(&self) -> &QoSCode {
        &self.qos
    }

    pub(crate) fn retain(&self) -> bool {
        self.retain
    }
}
#[allow(dead_code)]
impl PublishFixedHeader {
    pub(crate) fn new(fixed_header_reserved_flags: PublishFixedHeaderReservedFlags) -> Self {
        PublishFixedHeader {
            control_packet_type: ControlPacketType::Publish,
            fixed_header_reserved_flags,
            remaining_len: 0,
        }
    }

    pub(crate) fn self_create(
        control_packet_type: ControlPacketType,
        fixed_header_reserved_flags: PublishFixedHeaderReservedFlags,
        remaining_len: u32,
    ) -> PublishFixedHeader {
        PublishFixedHeader {
            control_packet_type,
            fixed_header_reserved_flags,
            remaining_len,
        }
    }

    pub(crate) fn control_packet_type(&self) -> &ControlPacketType {
        &self.control_packet_type
    }

    pub(crate) fn fixed_header_reserved_flags(&self) -> &PublishFixedHeaderReservedFlags {
        &self.fixed_header_reserved_flags
    }

    pub(crate) fn remaining_length(&self) -> u32 {
        self.remaining_len
    }
}
