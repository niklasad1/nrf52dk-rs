//! GPIO
//! Simplest possible implementation
//! Note, that the GPIOs are active low

use tock_regs::regs::ReadWrite;

const GPIO_BASE: usize = 0x5000_0000;
#[allow(dead_code)]
const NUM_GPIOTE: usize = 8;
#[allow(dead_code)]
const GPIOTE_BASE: usize = 0x4000_6000;
const NUMBER_PINS: usize = 32;

/// GPIO Singleton, use this as reference to access drivers
pub static GPIO: Gpio = Gpio::new();

/// GPIO Registers
#[repr(C)]
pub struct GpioRegisters {
    /// Reserved
    _reserved1: [u32; 321],
    /// Write GPIO port
    /// - Address: 0x504 - 0x508
    out: ReadWrite<u32, Out::Register>,
    /// Set individual bits in GPIO port
    /// - Address: 0x508 - 0x50C
    outset: ReadWrite<u32, OutSet::Register>,
    /// Clear individual bits in GPIO port
    /// - Address: 0x50C - 0x510
    outclr: ReadWrite<u32, OutClr::Register>,
    /// Read GPIO Port
    /// - Address: 0x510 - 0x514
    in_: ReadWrite<u32, In::Register>,
    /// Direction of GPIO pins
    /// - Address: 0x514 - 0x518
    dir: ReadWrite<u32, Dir::Register>,
    /// DIR set register
    /// - Address: 0x518 - 0x51C
    dirset: ReadWrite<u32, DirSet::Register>,
    /// DIR clear register
    /// - Address: 0x51C - 0x520
    dirclr: ReadWrite<u32, DirClr::Register>,
    /// Latch register indicating what GPIO pins that have met the criteria set in the
    /// PIN_CNF[n].SENSE
    /// - Address: 0x520 - 0x524
    latch: ReadWrite<u32, Latch::Register>,
    /// Select between default DETECT signal behaviour and LDETECT mode
    /// - Address: 0x524 - 0x528
    detect_mode: ReadWrite<u32, DetectMode::Register>,
    /// Reserved
    _reserved2: [u32; 118],
    /// Configuration of GPIO pins
    pin_cnf: [ReadWrite<u32, PinConfig::Register>; 32],
}

/// Gpio
register_bitfields! [u32,
    /// Write GPIO port
    Out [
        /// Pin[n], each bit correspond to a pin 0 to 31
        /// 0 - Low, Pin driver is low
        /// 1 - High, Pin driver is high
        PIN OFFSET(0) NUMBITS(32)
    ],
    /// Set individual bits in GPIO port
    OutSet [
        /// Pin[n], each bit correspond to a pin 0 to 31
        /// 0 - Low
        /// 1 - High
        /// Writing a '1' sets the pin high
        /// Writing a '0' has no effect
        PIN OFFSET(0) NUMBITS(32)
    ],
    /// Clear individual bits in GPIO port
    OutClr [
        /// Pin[n], each bit correspond to a pin 0 to 31
        /// 0 - Low
        /// 1 - High
        /// Writing a '1' sets the pin low
        /// Writing a '0' has no effect
        PIN OFFSET(0) NUMBITS(32)
    ],
    /// Read GPIO port
    In [
        /// Pin[n], each bit correspond to a pin 0 to 31
        /// 0 - Low
        /// 1 - High
        PIN OFFSET(0) NUMBITS(32)
    ],
    /// Direction of GPIO pins
    Dir [
        /// 0 - Pin set as input
        /// 1 - Pin set as output
        PIN OFFSET(0) NUMBITS(32)
    ],
    /// Configure direction of individual GPIO pins as output
    DirSet [
        /// Pin[n], each bit correspond to a pin 0 to 31
        /// 0 - Pin set as input
        /// 1 - Pin set as output
        /// Write: writing a '1' sets pin to output
        /// Writing a '0' has no effect
        PIN OFFSET(0) NUMBITS(32)
    ],
    /// Configure direction of individual GPIO pins as input
    DirClr [
        /// Pin[n], each bit correspond to a pin 0 to 31
        /// 0 - Pin set as input
        /// 1 - Pin set as output
        /// Write: writing a '1' sets pin to input
        /// Writing a '0' has no effect
        PIN OFFSET(0) NUMBITS(32)
    ],
    /// Latch register indicating what GPIO pins that have met the criteria set in the
    /// PIN_CNF[n].SENSE registers
    Latch [
        /// Pin[n], each bit correspond to a pin 0 to 31
        /// 0 - NotLatched
        /// 1 - Latched
        PIN OFFSET(0) NUMBITS(32)
    ],
    /// Select between default DETECT signal behaviour and LDETECT mode
    DetectMode [
        /// 0 - NotLatched
        /// 1 - Latched
        DETECTMODE OFFSET(0) NUMBITS(1) [
            DEFAULT = 0,
            LDDETECT = 1
        ]
    ],
    /// Configuration of GPIO pins
    /// Pin[n], each bit correspond to a pin 0 to 31
    PinConfig [
        /// Pin direction. Same physical register as DIR register
        DIR OFFSET(0) NUMBITS(1) [
            Input = 0,
            Output = 1
        ],
        /// Connect or disconnect input buffer
        INPUT OFFSET(1) NUMBITS(1) [
            Connect = 0,
            Disconnect = 1
        ],
        /// Pull configuration
        PULL OFFSET(2) NUMBITS(2) [
            Disabled = 0,
            Pulldown = 1,
            Pullup = 3
        ],
        /// Drive configuration
        DRIVE OFFSET(8) NUMBITS(3) [
            /// Standard '0', standard '1'
            S0S1 = 0,
            /// High drive '0', standard '1'
            H0S1 = 1,
            /// Standard '0', high drive '1
            S0H1 = 2,
            /// High drive '0', high 'drive '1'
            H0H1 = 3,
            /// Disconnect '0' standard '1' (normally used for wired-or connections)
            D0S1 = 4,
            /// Disconnect '0', high drive '1' (normally used for wired-or connections)
            D0H1 = 5,
            /// Standard '0'. disconnect '1' (normally used for wired-and connections)
            S0D1 = 6,
            /// High drive '0', disconnect '1' (normally used for wired-and connections)
            H0D1 = 7
        ],
        /// Pin sensing mechanism
        SENSE OFFSET(16) NUMBITS(2) [
            /// Disabled
            Disabled = 0,
            /// Sense for high level
            High = 2,
            /// Sense for low level
            Low = 3
        ]
    ]
];

/// GPIO Handle
pub struct Gpio {
    registers: *const GpioRegisters,
}

impl Gpio {
    /// GPIO Constructor
    pub const fn new() -> Self {
        Gpio {
            registers: GPIO_BASE as *const GpioRegisters,
        }
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
