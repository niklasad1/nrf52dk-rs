//! GPIO
//! Simplest possible implementation
//! Note, that the GPIOs are active low

use vcell::VolatileCell;

const GPIO_BASE: usize = 0x50000000;
#[allow(dead_code)]
const NUM_GPIOTE: usize = 8;
#[allow(dead_code)]
const GPIOTE_BASE: usize = 0x40006000;
const NUMBER_PINS: usize = 32;

/// GPIO Singleton, use this as reference to access drivers
pub static GPIO: Gpio = Gpio::new();

/// GPIO Registers
#[repr(C)]
struct GpioRegisters {
    _reserved1: [u32; 321],
    pub out: VolatileCell<u32>,
    pub outset: VolatileCell<u32>,
    pub outclr: VolatileCell<u32>,
    pub in_: VolatileCell<u32>,
    pub dir: VolatileCell<u32>,
    pub dirset: VolatileCell<u32>,
    pub dirclr: VolatileCell<u32>,
    _reserved2: [u32; 120],
    pub pin_cnf: [VolatileCell<u32>; NUMBER_PINS],
}

/// GPIO Handle
pub struct Gpio {
    registers: *const GpioRegisters,
}

impl Gpio {
    /// GPIO Constructor
    pub const fn new() -> Self {
        Gpio { registers: GPIO_BASE as *const GpioRegisters }
    }

    /// Configure a pin as output
    pub unsafe fn make_output(&self, pin: u32) {
        assert!(pin as usize <= NUMBER_PINS);
        let regs = &*self.registers;
        regs.dirset.set(1 << pin);
        regs.pin_cnf[pin as usize].set(1);
    }

    /// Configure a pin as input
    pub unsafe fn make_input(&self, pin: u32) {
        assert!(pin as usize <= NUMBER_PINS);
        let regs = &*self.registers;
        regs.dirclr.set(1 << pin);
        regs.pin_cnf[pin as usize].set(0);
    }

    /// Set pin high
    pub unsafe fn set(&self, pin: u32) {
        assert!(pin as usize <= NUMBER_PINS);
        let regs = &*self.registers;
        regs.outset.set(1 << pin);
    }

    /// Set pin low
    pub unsafe fn clear(&self, pin: u32) {
        assert!(pin as usize <= NUMBER_PINS);
        let regs = &*self.registers;
        regs.outclr.set(1 << pin);
    }

    /// Toggle pin
    pub unsafe fn toggle(&self, pin: u32) {
        assert!(pin as usize <= NUMBER_PINS);
        let regs = &*self.registers;
        regs.out.set((1 << pin) ^ regs.out.get());
    }
}

unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
