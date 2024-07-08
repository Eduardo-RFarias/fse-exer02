use crc::{Crc, CRC_16_IBM_SDLC};

const CRC16: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

pub fn hash(data: &[u8]) -> u16 {
    CRC16.checksum(data)
}
