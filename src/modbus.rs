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

pub fn create_modbus_message(command: &Command, data: &[u8]) -> Vec<u8> {
    let mut message = Vec::with_capacity(5 + data.len());

    message.push(command.code);
    message.push(command.subcode);
    message.extend(data);

    let crc = crc16::hash(&message);
    message.extend(&crc.to_be_bytes());

    message
}
