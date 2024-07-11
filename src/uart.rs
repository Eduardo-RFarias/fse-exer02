use crate::{crc16, modbus};
use rppal::uart::{Error, Parity, Queue, Uart as RppalUart};
use std::{thread, time::Duration};

pub struct Uart {
    uart: RppalUart,
}

impl Uart {
    pub fn new() -> Result<Self, Error> {
        let mut uart = RppalUart::new(9600, Parity::None, 8, 1)?;

        uart.set_read_mode(0, Duration::from_millis(100))?;

        Ok(Uart { uart })
    }

    pub fn clear_rx_tx(&mut self) -> Result<(), Error> {
        self.uart.flush(Queue::Both)
    }

    pub fn write_int(&mut self, data: i32) -> Result<i32, Error> {
        let message = modbus::create_modbus_message(&modbus::WRITE_INT, &data.to_le_bytes());
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut response = [0; 9];
        self.uart.read(&mut response)?;

        let response = modbus::extract_modbus_message(&response).unwrap();

        Ok(i32::from_be_bytes(response))
    }

    pub fn write_float(&mut self, data: f32) -> Result<f32, Error> {
        let message = modbus::create_modbus_message(&modbus::WRITE_FLOAT, &data.to_le_bytes());
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut response = [0; 9];
        self.uart.read(&mut response)?;

        let response = modbus::extract_modbus_message(&response).unwrap();

        Ok(f32::from_be_bytes(response))
    }

    pub fn write_string(&mut self, data: &str) -> Result<String, Error> {
        // string message is 1 byte for length and n bytes for data
        let mut message = Vec::with_capacity(1 + data.len());
        message.push(data.len() as u8);
        message.extend(data.as_bytes());

        let message = modbus::create_modbus_message(&modbus::WRITE_STRING, &message);
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut header = vec![0; 4];
        self.uart.read(&mut header)?;

        let mut body = vec![0; header[3] as usize];
        self.uart.read(&mut body)?;

        let mut crc = vec![0; 2];
        self.uart.read(&mut crc)?;

        let response: Vec<u8> = [header, body.clone()].concat();

        let crc = u16::from_le_bytes([crc[0], crc[1]]);
        let expected_crc = crc16::hash(&response);

        if crc != expected_crc {
            return Err(Error::InvalidValue);
        }

        Ok(String::from_utf8(body).unwrap())
    }

    pub fn read_int(&mut self) -> Result<i32, Error> {
        let message = modbus::create_modbus_message(&modbus::READ_INT, &[]);
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut response = [0; 9];
        self.uart.read(&mut response)?;

        let response = modbus::extract_modbus_message(&response).unwrap();

        Ok(i32::from_be_bytes(response))
    }

    pub fn read_float(&mut self) -> Result<f32, Error> {
        let message = modbus::create_modbus_message(&modbus::READ_FLOAT, &[]);
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut response = [0; 9];
        self.uart.read(&mut response)?;

        let response = modbus::extract_modbus_message(&response).unwrap();

        Ok(f32::from_be_bytes(response))
    }

    pub fn read_string(&mut self) -> Result<String, Error> {
        let message = modbus::create_modbus_message(&modbus::READ_STRING, &[]);
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut header = vec![0; 4];
        self.uart.read(&mut header)?;

        let mut body = vec![0; header[3] as usize];
        self.uart.read(&mut body)?;

        let mut crc = vec![0; 2];
        self.uart.read(&mut crc)?;

        let response: Vec<u8> = [header, body.clone()].concat();

        let crc = u16::from_le_bytes([crc[0], crc[1]]);
        let expected_crc = crc16::hash(&response);

        if crc != expected_crc {
            return Err(Error::InvalidValue);
        }

        Ok(String::from_utf8(body).unwrap())
    }
}
