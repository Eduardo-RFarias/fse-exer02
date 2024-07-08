use std::{thread, time::Duration};

use crate::modbus;
use rppal::uart::{Error, Parity, Queue, Uart as RppalUart};

pub struct Uart {
    uart: RppalUart,
}

impl Uart {
    pub fn new() -> Result<Self, Error> {
        let uart = RppalUart::new(9600, Parity::None, 8, 1)?;
        Ok(Uart { uart })
    }

    pub fn clear_rx_tx(&mut self) -> Result<(), Error> {
        self.uart.flush(Queue::Both)
    }

    pub fn write_int(&mut self, data: i32) -> Result<i32, Error> {
        let message = modbus::create_modbus_message(&modbus::WRITE_INT, &data.to_be_bytes());
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut response = [0; 4];
        self.uart.read(&mut response)?;

        Ok(i32::from_be_bytes(response))
    }

    pub fn write_float(&mut self, data: f32) -> Result<f32, Error> {
        let message = modbus::create_modbus_message(&modbus::WRITE_FLOAT, &data.to_be_bytes());
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut response = [0; 4];
        self.uart.read(&mut response)?;

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

        let mut response = vec![0; data.len()];
        self.uart.read(&mut response)?;

        Ok(String::from_utf8(response).map_err(|_| Error::InvalidValue)?)
    }

    pub fn read_int(&mut self) -> Result<i32, Error> {
        let message = modbus::create_modbus_message(&modbus::READ_INT, &[]);
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut response = [0; 4];
        self.uart.read(&mut response)?;

        Ok(i32::from_be_bytes(response))
    }

    pub fn read_float(&mut self) -> Result<f32, Error> {
        let message = modbus::create_modbus_message(&modbus::READ_FLOAT, &[]);
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut response = [0; 4];
        self.uart.read(&mut response)?;

        Ok(f32::from_be_bytes(response))
    }

    pub fn read_string(&mut self) -> Result<String, Error> {
        let message = modbus::create_modbus_message(&modbus::READ_STRING, &[]);
        self.uart.write(&message)?;

        thread::sleep(Duration::from_millis(100));

        let mut response = vec![0; 1];
        self.uart.read(&mut response)?;

        let length = response[0] as usize;
        let mut response = vec![0; length];
        self.uart.read(&mut response)?;

        Ok(String::from_utf8(response).map_err(|_| Error::InvalidValue)?)
    }
}
