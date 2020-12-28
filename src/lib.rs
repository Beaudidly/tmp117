#![no_std]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


use embedded_hal as hal;

const DEFAULT_ADDRESS: u8 = 0x48u8;
const TEMP_ADDR: u8 = 0x0u8;
const TEMP_LENGTH: usize = 0x2;

/// TMP1117 Driver
pub struct TMP117<I2C: hal::blocking::i2c::WriteRead> {
    com: I2C,
    addr: u8,
}

impl<I2C: hal::blocking::i2c::WriteRead> TMP117<I2C> {
    pub fn new(i2c: I2C, addr: u8) -> TMP117<I2C>
    {
        TMP117{com: i2c, addr: addr}
    }

    pub fn new_default(i2c: I2C) -> TMP117<I2C> {
        TMP117::new(i2c, DEFAULT_ADDRESS)
    }

    pub fn read(&mut self) -> Result<f32, I2C::Error> {
        let mut data: [u8; TEMP_LENGTH] = [0; TEMP_LENGTH];
        self.com.write_read(self.addr, &[TEMP_ADDR], &mut data)?;

        let count = ((data[0] as u32) << 8) | (data[1] as u32);
        let C = ((count as f32) * 7.8125) / 1000.0;

        return Ok(C);
    }
}
