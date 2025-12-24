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
use crate::protocol::mqtt_protocol_error::MqttProtocolError;
use crate::protocol::mqtt4::fixed_header_parser::fixed_header::FixedHeader;
use crate::protocol::mqtt4::variable_header_parser::mqtt_variable_header_codec::MqttVariableHeaderDecoder;
use crate::utils::radix::radix_handler;
use crate::utils::utf::utf_8_handler;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct ConnectVariableHeader {
    protocol_level: u8,
    connect_flags: ConnectFlags,
    keep_alive: u16,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct ConnectFlags {
    username_flag: bool,
    password_flag: bool,
    will_retain: bool,
    will_qos: u8,
    will_flag: bool,
    clean_session: bool,
}

#[allow(dead_code)]
impl ConnectFlags {
    pub fn new(
        user_name_flag: bool,
        password_flag: bool,
        will_retain: bool,
        will_qos: u8,
        will_flag: bool,
        clean_session: bool,
    ) -> Result<Self, MqttProtocolError> {
        let this = Self {
            username_flag: user_name_flag,
            password_flag,
            will_retain,
            will_qos,
            will_flag,
            clean_session,
        };

        this.verify()?;

        Ok(this)
    }

    fn verify(&self) -> Result<(), MqttProtocolError> {
        self.verify_state_when_user_name_flag_is_0()?;
        self.verify_will_qos()?;
        self.verify_state_when_will_flag_is_0()?;
        Ok(())
    }

    fn verify_state_when_user_name_flag_is_0(&self) -> Result<(), MqttProtocolError> {
        if !self.username_flag && self.password_flag {
            return Err(MqttProtocolError::MalformedPacket);
        }
        Ok(())
    }

    fn verify_will_qos(&self) -> Result<(), MqttProtocolError> {
        if self.will_qos > 2 {
            return Err(MqttProtocolError::InvalidWillQoS(self.will_qos));
        }
        Ok(())
    }

    fn verify_state_when_will_flag_is_0(&self) -> Result<(), MqttProtocolError> {
        if !self.will_flag {
            if self.will_qos != 0 {
                return Err(MqttProtocolError::MalformedPacket);
            }
            if self.will_retain {
                return Err(MqttProtocolError::MalformedPacket);
            }
        }
        Ok(())
    }

    pub fn username_flag(&self) -> bool {
        self.username_flag
    }

    pub fn password_flag(&self) -> bool {
        self.password_flag
    }

    pub fn will_retain(&self) -> bool {
        self.will_retain
    }

    pub fn will_qos(&self) -> u8 {
        self.will_qos
    }

    pub fn will_flag(&self) -> bool {
        self.will_flag
    }

    pub fn clean_session(&self) -> bool {
        self.clean_session
    }
}

#[allow(dead_code)]
impl ConnectVariableHeader {
    pub fn new(protocol_level: u8, connect_flags: ConnectFlags, keep_alive: u16) -> Self {
        Self {
            protocol_level,
            connect_flags,
            keep_alive,
        }
    }

    pub fn protocol_level(&self) -> u8 {
        self.protocol_level
    }

    pub fn connect_flags(&self) -> &ConnectFlags {
        &self.connect_flags
    }

    pub fn keep_alive(&self) -> u16 {
        self.keep_alive
    }
}

impl MqttVariableHeaderDecoder for ConnectVariableHeader {
    fn decode(
        _fixed_header: &FixedHeader,
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnectVariableHeader, MqttProtocolError> {
        Self::parse(bytes)
    }
}

#[allow(dead_code)]
impl ConnectVariableHeader {
    fn parse(bytes: &mut impl ByteOperations) -> Result<ConnectVariableHeader, MqttProtocolError> {
        Self::verify_protocol_name(bytes)?;
        let protocol_level = Self::verify_and_return_protocol_level(bytes)?;
        let connect_flags = Self::parser_connect_flags(bytes)?;
        let keep_alive = Self::parse_keep_alive(bytes)?;

        Ok(ConnectVariableHeader {
            protocol_level,
            connect_flags,
            keep_alive,
        })
    }
    fn verify_protocol_name(bytes: &mut impl ByteOperations) -> Result<(), MqttProtocolError> {
        let protocol_name = utf_8_handler::read(bytes)?;
        if protocol_name != "MQTT" {
            return Err(MqttProtocolError::ProtocolNameError(protocol_name));
        }
        Ok(())
    }
    fn verify_and_return_protocol_level(
        bytes: &mut impl ByteOperations,
    ) -> Result<u8, MqttProtocolError> {
        let protocol_level = bytes
            .read_a_byte()
            .ok_or(MqttProtocolError::PacketTooShort)?;
        if protocol_level != 4 {
            return Err(MqttProtocolError::ProtocolLevelNoSupport(protocol_level));
        }
        Ok(protocol_level)
    }

    fn parser_connect_flags(
        bytes: &mut impl ByteOperations,
    ) -> Result<ConnectFlags, MqttProtocolError> {
        let connect_flags_byte = bytes
            .read_a_byte()
            .ok_or(MqttProtocolError::PacketTooShort)?;

        let user_name_flag = ConnectVariableHeader::parse_user_name_flag(connect_flags_byte);
        let password_flag = ConnectVariableHeader::parse_password_flag(connect_flags_byte);
        let will_retain = ConnectVariableHeader::parse_will_retain(connect_flags_byte);
        let will_qos = ConnectVariableHeader::parse_qos(connect_flags_byte);
        let will_flag = ConnectVariableHeader::parse_will_flag(connect_flags_byte);
        let clean_session = ConnectVariableHeader::parse_clean_session(connect_flags_byte);
        ConnectVariableHeader::verify_reserved_bit(connect_flags_byte)?;

        let connect_flags = ConnectFlags::new(
            user_name_flag,
            password_flag,
            will_retain,
            will_qos,
            will_flag,
            clean_session,
        )?;

        Ok(connect_flags)
    }

    fn parse_user_name_flag(connect_flags_byte: u8) -> bool {
        (connect_flags_byte & 0b1000_0000) != 0
    }

    fn parse_password_flag(connect_flags_byte: u8) -> bool {
        (connect_flags_byte & 0b0100_0000) != 0
    }

    fn parse_will_retain(connect_flags_byte: u8) -> bool {
        (connect_flags_byte & 0b0010_0000) != 0
    }

    fn parse_qos(connect_flags_byte: u8) -> u8 {
        (connect_flags_byte & 0b0001_1000) >> 3
    }

    fn parse_will_flag(connect_flags_byte: u8) -> bool {
        (connect_flags_byte & 0b0000_0100) != 0
    }

    fn parse_clean_session(connect_flags_byte: u8) -> bool {
        (connect_flags_byte & 0b0000_0010) != 0
    }

    fn verify_reserved_bit(connect_flags_byte: u8) -> Result<(), MqttProtocolError> {
        let reserved_bit = connect_flags_byte & 0b0000_0001;
        if reserved_bit != 0 {
            return Err(MqttProtocolError::MalformedPacket);
        }
        Ok(())
    }

    fn parse_keep_alive(bytes: &mut impl ByteOperations) -> Result<u16, MqttProtocolError> {
        let length_bytes = bytes.read_bytes(2);
        let keep_alive = radix_handler::be_bytes_to_u16(length_bytes.as_slice())?;
        Ok(keep_alive)
    }
}

#[cfg(test)]
mod connect_variable_header_tests {
    use crate::byte_adapter::byte_operations::ByteOperations;
    use crate::protocol::mqtt_protocol_error::MqttProtocolError;
    use crate::protocol::mqtt4::variable_header_parser::connect::ConnectVariableHeader;
    use crate::utils::utf::utf_8_handler::write;
    use bytes::BytesMut;

    fn create_test_connect_flags_byte(connect_flags_byte: u8) -> u8 {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_a_byte(connect_flags_byte);

        bytes_mut.read_a_byte().unwrap()
    }

    #[test]
    fn connect_should_parse_connect_variable_header() {
        let mut bytes_mut = BytesMut::new();
        write(&mut bytes_mut, "MQTT").unwrap();
        bytes_mut.write_a_byte(0b0000_0100); // protocol level 4
        bytes_mut.write_a_byte(0b1100_1110); // connect flags
        bytes_mut.write_a_byte(0x00); // keep alive MSB
        bytes_mut.write_a_byte(0x3C); // keep alive LSB (60 seconds)

        let connect_variable_header = ConnectVariableHeader::parse(&mut bytes_mut).unwrap();

        assert_eq!(connect_variable_header.protocol_level, 4);
        assert!(connect_variable_header.connect_flags.username_flag());
        assert!(connect_variable_header.connect_flags.password_flag());
        assert!(!connect_variable_header.connect_flags.will_retain());
        assert_eq!(connect_variable_header.connect_flags.will_qos(), 1);
        assert!(connect_variable_header.connect_flags.will_flag());
        assert!(connect_variable_header.connect_flags.clean_session());
        assert_eq!(connect_variable_header.keep_alive, 60);
    }

    #[test]
    fn connect_should_fail_when_packet_too_short() {
        let mut bytes_mut = BytesMut::new();
        // Incomplete variable header (only protocol name)
        write(&mut bytes_mut, "MQTT").unwrap();

        let result = ConnectVariableHeader::parse(&mut bytes_mut);
        assert!(result.is_err());
        assert!(matches!(result, Err(MqttProtocolError::PacketTooShort)));
    }

    #[test]
    fn connect_can_allowed_valid_protocol_name() {
        let mut bytes_mut = BytesMut::new();
        write(&mut bytes_mut, "MQTT").unwrap();
        ConnectVariableHeader::verify_protocol_name(&mut bytes_mut).unwrap();
    }
    #[test]
    fn connect_can_not_allowed_invalid_protocol_name() {
        let mut bytes_mut = BytesMut::new();
        let invalid_name = "hello";
        write(&mut bytes_mut, invalid_name).unwrap();
        let result = ConnectVariableHeader::verify_protocol_name(&mut bytes_mut);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(MqttProtocolError::ProtocolNameError(_invalid_name))
        ));
    }
    #[test]
    fn connect_should_parse_protocol_level() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_a_byte(0b0000_0100);
        let protocol_level =
            ConnectVariableHeader::verify_and_return_protocol_level(&mut bytes_mut).unwrap();
        assert_eq!(protocol_level, 4);
    }

    #[test]
    fn connect_should_not_allow_invalid_protocol_level() {
        let mut bytes_mut = BytesMut::new();
        let invalid_level = 5;
        bytes_mut.write_a_byte(invalid_level);
        let result = ConnectVariableHeader::verify_and_return_protocol_level(&mut bytes_mut);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(MqttProtocolError::ProtocolLevelNoSupport(level)) if level == invalid_level
        ));
    }
    #[test]
    fn connect_should_parse_user_name_flag_is_true() {
        // username flag is the 7th bit in the connect flags byte
        let connect_flags_byte: u8 = 0b1000_0000; // username flag set to 1
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let user_name_flag = ConnectVariableHeader::parse_user_name_flag(connect_flags);

        assert!(user_name_flag);
    }

    #[test]
    fn connect_should_parse_user_name_flag_is_false() {
        // username flag is the 7th bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_0000; // username flag set to 0
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let user_name_flag = ConnectVariableHeader::parse_user_name_flag(connect_flags);

        assert!(!user_name_flag);
    }

    #[test]
    fn connect_should_parse_password_flag_is_true() {
        // password flag is the 6th bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0100_0000; // password flag set to 1
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let password_flag = ConnectVariableHeader::parse_password_flag(connect_flags);

        assert!(password_flag);
    }

    #[test]
    fn connect_should_parse_password_flag_is_false() {
        // password flag is the 6th bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_0000; // password flag set to 0
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let password_flag = ConnectVariableHeader::parse_password_flag(connect_flags);

        assert!(!password_flag);
    }

    #[test]
    fn connect_should_parse_will_retain_is_true() {
        // will retain flag is the 5th bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0010_0000; // will retain flag set to 1
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let will_retain_flag = ConnectVariableHeader::parse_will_retain(connect_flags);

        assert!(will_retain_flag);
    }

    #[test]
    fn connect_should_parse_will_retain_is_false() {
        // will retain flag is the 5th bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_0000; // will retain flag set to 0
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let will_retain_flag = ConnectVariableHeader::parse_will_retain(connect_flags);

        assert!(!will_retain_flag);
    }

    #[test]
    fn connect_should_parse_will_qos_zero() {
        // will qos is the 4th and 3rd bits in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_0000; // will qos set to 0 (00)
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let will_qos = (connect_flags & 0b0001_1000) >> 3;

        assert_eq!(will_qos, 0);
    }

    #[test]
    fn connect_should_parse_will_qos_one() {
        // will qos is the 4th and 3rd bits in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_1000; // will qos set to 1 (01)
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let will_qos = (connect_flags & 0b0001_1000) >> 3;

        assert_eq!(will_qos, 1);
    }

    #[test]
    fn connect_should_parse_will_qos_two() {
        // will qos is the 4th and 3rd bits in the connect flags byte
        let connect_flags_byte: u8 = 0b0001_0000; // will qos set to 2 (10)
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let will_qos = (connect_flags & 0b0001_1000) >> 3;

        assert_eq!(will_qos, 2);
    }

    #[test]
    fn connect_should_parse_will_flag_is_true() {
        // will flag is the 2nd bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_0100; // will flag set to 1
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let will_flag = ConnectVariableHeader::parse_will_flag(connect_flags);

        assert!(will_flag);
    }

    #[test]
    fn connect_should_parse_will_flag_is_false() {
        // will flag is the 2nd bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_0000; // will flag set to 0
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let will_flag = ConnectVariableHeader::parse_will_flag(connect_flags);

        assert!(!will_flag);
    }
    #[test]
    fn connect_should_parse_clean_session_is_true() {
        // clean session flag is the 1st bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_0010; // clean session flag set to 1
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let clean_session_flag = ConnectVariableHeader::parse_clean_session(connect_flags);
        assert!(clean_session_flag);
    }

    #[test]
    fn connect_should_parse_clean_session_is_false() {
        // clean session flag is the 1st bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_0000; // clean session flag set to 0
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let clean_session_flag = ConnectVariableHeader::parse_clean_session(connect_flags);
        assert!(!clean_session_flag);
    }

    #[test]
    fn connect_should_verify_reserved_bit_can_not_allow_set_1() {
        // reserved bit is the 0th bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_0001; // reserved bit set to 1
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let reserved_bit = ConnectVariableHeader::verify_reserved_bit(connect_flags);
        assert!(reserved_bit.is_err());
        assert!(matches!(
            reserved_bit,
            Err(MqttProtocolError::MalformedPacket)
        ));
    }

    #[test]
    fn connect_should_verify_reserved_bit_is_zero_pass() {
        // reserved bit is the 0th bit in the connect flags byte
        let connect_flags_byte: u8 = 0b0000_0000; // reserved bit set to 0
        let connect_flags = create_test_connect_flags_byte(connect_flags_byte);
        let reserved_bit = ConnectVariableHeader::verify_reserved_bit(connect_flags);
        assert!(reserved_bit.is_ok());
    }

    #[test]
    fn connect_should_parse_connect_flags() {
        // Example connect flags byte: 0b1101_1010
        let connect_flags_byte: u8 = 0b1100_1110;
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_a_byte(connect_flags_byte);

        let connect_flags = ConnectVariableHeader::parser_connect_flags(&mut bytes_mut).unwrap();

        assert!(connect_flags.username_flag()); // 7th bit is 1
        assert!(connect_flags.password_flag()); // 6th bit is 1
        assert!(!connect_flags.will_retain()); // 5th bit is 0
        assert_eq!(connect_flags.will_qos(), 1); // 4th and 3rd bits are 10
        assert!(connect_flags.will_flag()); // 2nd bit is 1
        assert!(connect_flags.clean_session()); // 1st bit is 1
    }

    #[test]
    fn connect_should_parse_keep_alive() {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.write_a_byte(0x00);
        bytes_mut.write_a_byte(0x3C); // 60 seconds

        let keep_alive = ConnectVariableHeader::parse_keep_alive(&mut bytes_mut).unwrap();
        assert_eq!(keep_alive, 60);
    }
}

#[cfg(test)]
mod connect_flags_verify_tests {
    use crate::protocol::mqtt4::variable_header_parser::connect::ConnectFlags;

    #[test]
    fn will_flag_false_then_will_qos_is_0_and_will_retain_must_be_false() {
        let will_flag = false;
        let will_qos = 0;
        let will_retain = false;
        let result = ConnectFlags::new(false, false, will_retain, will_qos, will_flag, false);

        assert!(result.is_ok());
    }
    #[test]
    fn will_flag_false_then_will_qos_is_not_0_should_return_error() {
        let will_flag = false;
        let will_qos = 1;
        let will_retain = false;
        let result = ConnectFlags::new(false, false, will_retain, will_qos, will_flag, false);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(crate::protocol::mqtt_protocol_error::MqttProtocolError::MalformedPacket)
        ))
    }

    #[test]
    fn will_flag_false_then_will_retain_is_true_should_return_error() {
        let will_flag = false;
        let will_qos = 0;
        let will_retain = true;
        let result = ConnectFlags::new(false, false, will_retain, will_qos, will_flag, false);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(crate::protocol::mqtt_protocol_error::MqttProtocolError::MalformedPacket)
        ))
    }

    #[test]
    fn user_name_flag_false_then_password_flag_must_be_false() {
        let user_name_flag = false;
        let password_flag = false;
        let result = ConnectFlags::new(user_name_flag, password_flag, false, 0, false, false);

        assert!(result.is_ok());
    }

    #[test]
    fn user_name_flag_false_then_password_flag_is_true_should_return_error() {
        let user_name_flag = false;
        let password_flag = true;
        let result = ConnectFlags::new(user_name_flag, password_flag, false, 0, false, false);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(crate::protocol::mqtt_protocol_error::MqttProtocolError::MalformedPacket)
        ))
    }

    #[test]
    fn will_qos_zero_one_two_should_pass() {
        for will_qos in 0..=2 {
            let result = ConnectFlags::new(false, false, false, will_qos, true, false);
            assert!(result.is_ok());
        }
    }
    #[test]
    fn will_qos_more_than_2_should_return_error() {
        let will_qos = 3;
        let result = ConnectFlags::new(false, false, false, will_qos, false, false);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(crate::protocol::mqtt_protocol_error::MqttProtocolError::InvalidWillQoS(qos)) if qos == will_qos
        ))
    }
}
