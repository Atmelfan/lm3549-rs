extern crate linux_embedded_hal as hal;
extern crate lm3549;

use hal::I2cdev;
use lm3549::LM3549;

fn main() {
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
}
