use crate::crc16;

pub struct Command {
    code: u8,
    subcode: u8,
}

pub const READ_INT: Command = Command {
    code: 0x23,
    subcode: 0xA1,
};

pub const READ_FLOAT: Command = Command {
    code: 0x23,
    subcode: 0xA2,
};

pub const READ_STRING: Command = Command {
    code: 0x23,
    subcode: 0xA3,
};

pub const WRITE_INT: Command = Command {
    code: 0x16,
    subcode: 0xB1,
};

pub const WRITE_FLOAT: Command = Command {
    code: 0x16,
    subcode: 0xB2,
};

pub const WRITE_STRING: Command = Command {
    code: 0x16,
    subcode: 0xB3,
};

pub const TARGET_ADDRESS: u8 = 0x01;

pub fn create_modbus_message(command: &Command, data: &[u8]) -> Vec<u8> {
    let mut message = Vec::with_capacity(9 + data.len());

    message.push(TARGET_ADDRESS);
    message.push(command.code);
    message.push(command.subcode);
    message.extend(data);
    message.extend([0, 0, 8, 6]);

    let crc = crc16::hash(&message);
    message.extend(&crc.to_le_bytes());

    message
}

pub fn extract_modbus_message(message: &[u8; 9]) -> Result<[u8; 4], &str> {
    let _target_address = message[0];
    let _code = message[1];
    let _subcode = message[2];
    let data = &message[3..message.len() - 2];
    let crc = u16::from_le_bytes([message[message.len() - 2], message[message.len() - 1]]);

    let expected_crc = crc16::hash(&message[..message.len() - 2]);

    if crc != expected_crc {
        return Err("CRC mismatch");
    }

    if data.len() != 4 {
        return Err("Data length mismatch");
    }

    Ok([data[0], data[1], data[2], data[3]])
}
