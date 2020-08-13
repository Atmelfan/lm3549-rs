//! This is a platform agnostic Rust driver for the for LM3549 High Power Sequential LED Driver
//! using the [`embedded-hal`] traits.
//!
//! Datasheet: [LM3549](https://www.ti.com/lit/ds/symlink/lm3549.pdf)
//!
//! ## License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
//!    http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or
//!    http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! ### Contributing
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall
//! be dual licensed as above, without any additional terms or conditions.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

#![no_std]

extern crate byteorder;
extern crate embedded_hal as hal;

use hal::blocking::i2c;

mod registers;
use registers::*;

const LM3549_ADDR: u8 = 0x36;

/// LM3549 High Power Sequential LED Driver
pub struct LM3549<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> LM3549<I2C> {
    /// Create new LM3549 instance with default address
    pub fn new(i2c: I2C) -> Self {
        LM3549 {
            i2c,
            address: LM3549_ADDR,
        }
    }
}

impl<I2C, E> LM3549<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::Read<Error = E>,
{
    /// Read a register
    pub fn read(&mut self, register: Register) -> Result<u8, E> {
        let mut buf: [u8; 1] = [0x00];
        self.i2c.write(self.address, &[register as u8])?;
        self.i2c.read(self.address, &mut buf)?;
        Ok(buf[0])
    }

    /// Get active faults
    pub fn get_fault(&mut self) -> Result<Fault, E> {
        let x = self.read(Register::Fault)?;
        Ok(Fault(x))
    }

    /// Write a register
    pub fn write(&mut self, register: Register, value: u8) -> Result<(), E> {
        let buf = [register as u8, value];
        self.i2c.write(self.address, &buf)
    }

    /// Write a register
    pub fn write_bank(&mut self, bank: Bank, r: u16, g: u16, b: u16) -> Result<(), E> {
        let buf = [
            bank as u8,
            (r & 0xFF) as u8,
            ((r >> 8) & 0x03) as u8,
            (g & 0xFF) as u8,
            ((g >> 8) & 0x03) as u8,
            (b & 0xFF) as u8,
            ((b >> 8) & 0x03) as u8,
        ];
        self.i2c.write(self.address, &buf)
    }

    /// Select driver current settings bank
    pub fn select_bank(&mut self, bank: Bank) -> Result<(), E> {
        let sel: u8 = match bank {
            Bank::B0 => 0,
            Bank::B1 => 1,
            Bank::B2 => 2,
        };
        self.write(Register::BankSel, sel)
    }

    /// Set master fader (Ctrl.mfe must be set)
    pub fn set_fader(&mut self, fade: u8) -> Result<(), E> {
        self.write(Register::Fader, fade)
    }

    /// Set control register
    pub fn set_ctrl(&mut self, ctrl: Ctrl) -> Result<(), E> {
        self.write(Register::Ctrl, ctrl.0)
    }

    /// Set current limit register
    pub fn set_ilimit(&mut self, limit: Ilimit) -> Result<(), E> {
        self.write(Register::Ilimit, limit.0)
    }

    /// Set fault mask register
    pub fn set_fault_mask(&mut self, mask: FaultMask) -> Result<(), E> {
        self.write(Register::FaultMask, mask.0)
    }
}
