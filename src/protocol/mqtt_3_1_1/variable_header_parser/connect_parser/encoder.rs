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

use crate::protocol::mqtt_3_1_1::variable_header_parser::connect_parser::variable_header::ConnectVariableHeader;
use crate::protocol::mqtt_3_1_1::variable_header_parser::variable_header_codec::VariableHeaderEncoder;
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::utils::radix;

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
impl VariableHeaderEncoder for ConnectVariableHeader {
    fn encode(&self) -> Result<Vec<u8>, MqttProtocolError>
    where
        Self: Sized,
    {
        let mut bytes: Vec<u8> = Vec::new();

        // Protocol Name
        let encode_protocol_name_len =
            radix::radix_handler::u16_to_be_2_bytes(self.protocol_name().len())?.to_vec();
        let encode_protocol_name = self.protocol_name().as_bytes().to_vec();
        bytes.extend(encode_protocol_name_len);
        bytes.extend(encode_protocol_name);

        // Protocol Level
        bytes.push(self.protocol_level().as_u8());

        // Connect Flags
        bytes.push(self.connect_flags().encode()?);

        // Keep Alive
        let keep_alive = self.keep_alive().to_be_bytes().to_vec();
        bytes.extend(keep_alive);

        Ok(bytes)
    }
}
