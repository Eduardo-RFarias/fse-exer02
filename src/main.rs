mod crc16;
mod modbus;
mod uart;

use uart::Uart;

fn main() {
    let mut uart = Uart::new().unwrap();

    uart.clear_rx_tx().unwrap();

    let data = 123;
    uart.write_int(data).unwrap();
    println!("write_int: {}", data);

    let data = 123.456;
    uart.write_float(data).unwrap();
    println!("write_float: {}", data);

    let data = "Hello, World!";
    uart.write_string(data).unwrap();
    println!("write_string: {}", data);

    let response = uart.read_int().unwrap();
    println!("read_int: {}", response);

    let response = uart.read_float().unwrap();
    println!("read_float: {}", response);

    // ! Não funciona ainda
    // let response = uart.read_string().unwrap();
    // println!("read_string: {}", response);

    uart.clear_rx_tx().unwrap();
}
