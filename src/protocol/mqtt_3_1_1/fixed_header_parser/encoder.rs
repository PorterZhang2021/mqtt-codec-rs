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

use crate::protocol::common::remaining_length::remaining_length_parser;
use crate::protocol::mqtt_3_1_1::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt_3_1_1::fixed_header_parser::fixed_header_codec::FixedHeaderEncoder;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;

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
impl FixedHeaderEncoder for FixedHeader {
    fn encode(&mut self, remaining_length: u32) -> Result<Vec<u8>, MqttProtocolError>
    where
        Self: Sized,
    {
        self.set_remaining_length(remaining_length);
        let mut bytes: Vec<u8> = Vec::new();

        let encode_fixed_header_reserved_flags = self.fixed_header_reserved_flags().encode();
        bytes.push(encode_fixed_header_reserved_flags);

        // Remaining Length
        let encode_remaining_length = remaining_length_parser::encode(remaining_length)?;
        bytes.extend(encode_remaining_length);

        Ok(bytes)
    }
}
