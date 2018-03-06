//! Clock peripheral driver
//!
//! LFCLK - Low Frequency Clock Source:
//!     * 32.768 kHz RC oscillator (LFRC)
//!     * 32.768 kHz crystal oscillator (LFXO)
//!     * 32.768 kHz synthesized from HFCLK (LFSYNT)
//!
//! HFCLK - High Frequency Clock
//!     * 64 MHz internal oscillator (HFINT)
//!     * 64 MHz crystal oscillator (HFXO)
//!     * HFXO must be running the run the RADIO, NFC and calibration

use tock_registers::common::regs::{WriteOnly, ReadWrite, ReadOnly};

const CLOCK_BASE: usize = 0x40000000;

pub static mut CLOCK: Clock = Clock::new();

/// Low frequency clock source 
pub enum LowClockSource {
    RC = 0,
    XTAL = 1,
    SYNTH = 2,
}

/// High frequency clock source 
pub enum HighClockSource {
    RC = 0,
    XTAL = 1,
}

/// Clock struct
pub struct Clock {
    registers: *const ClockRegisters,
}

impl Clock {
    pub const fn new() -> Clock {
        Clock {
            registers: CLOCK_BASE as *const ClockRegisters
        }
    }

    // pub unsafe fn interrupt_enable(&self, interrupt: InterruptField) {
    //     let regs = &*self.registers;
    //     regs.intenset.set(interrupt as u32);
    // }
    //
    // pub unsafe fn interrupt_disable(&self, interrupt: InterruptField) {
    //     let regs = &*self.registers;
    //     regs.intenclr.set(interrupt as u32);
    // }

    pub unsafe fn high_start(&self) {
        let regs = &*self.registers;
        regs.tasks_hfclkstart.write(Control::ENABLE.val(1));
    }

    pub unsafe fn high_stop(&self) {
        let regs = &*self.registers;
        regs.tasks_hfclkstop.write(Control::ENABLE.val(1));
    }

    pub unsafe fn high_started(&self) -> bool {
        let regs = &*self.registers;
        regs.events_hfclkstarted.read(Status::READY) == 1
    }

    pub unsafe fn high_set_source(&self, clk_source: HighClockSource) {
        let regs = &*self.registers;
        match clk_source {
            HighClockSource::XTAL => regs.hfclkstat.write(HfClkStat::SRC::XTAL),
            HighClockSource::RC => regs.hfclkstat.write(HfClkStat::SRC::RC),
        }
    }

    pub unsafe fn high_running(&self) -> bool {
        let regs = &*self.registers;
        regs.hfclkstat.matches(HfClkStat::STATE::RUNNING)
    }

    pub unsafe fn low_start(&self) {
        let regs = &*self.registers;
        regs.tasks_lfclkstart.set(1);
    }

    pub unsafe fn low_stop(&self) {
        let regs = &*self.registers;
        regs.tasks_lfclkstop.write(Control::ENABLE::SET);
    }

    pub unsafe fn low_started(&self) -> bool {
        let regs = &*self.registers;
        regs.events_lfclkstarted.matches(Status::READY::SET)
    }

    pub unsafe fn low_running(&self) -> bool {
        let regs = &*self.registers;
        regs.lfclkstat.matches(LfClkStat::STATE::RUNNING)
    }

    pub unsafe fn low_set_source(&self, clk_source: LowClockSource) {
        let regs = &*self.registers;
        match clk_source {
            LowClockSource::RC => regs.lfclksrc.write(LfClkSrc::SRC::RC),
            LowClockSource::XTAL => regs.lfclksrc.write(LfClkSrc::SRC::XTAL),
            LowClockSource::SYNTH => regs.lfclksrc.write(LfClkSrc::SRC::SYNTH),
        }
    }
}

struct ClockRegisters {
    pub tasks_hfclkstart: WriteOnly<u32, Control::Register>, // 0x000
    pub tasks_hfclkstop: WriteOnly<u32, Control::Register>,  // 0x004
    pub tasks_lfclkstart: ReadWrite<u32, Control::Register>, // 0x008
    pub tasks_lfclkstop: WriteOnly<u32, Control::Register>,  // 0x00c
    pub tasks_cal: WriteOnly<u32, Control::Register>,        // 0x010
    pub tasks_ctstart: WriteOnly<u32, Control::Register>,    // 0x014
    pub tasks_ctstop: WriteOnly<u32, Control::Register>,     // 0x018
    _reserved1: [u32; 57],                                   // 0x018 - 0x100
    pub events_hfclkstarted: ReadOnly<u32, Status::Register>, // 0x100
    pub events_lfclkstarted: ReadOnly<u32, Status::Register>, // 0x104
    _reserverd2: u32,                                        // 0x108
    pub events_done: ReadOnly<u32, Status::Register>,        // 0x10c
    pub events_ctto: ReadOnly<u32, Status::Register>,        // 0x110
    _reserved3: [u32; 124],                                  // 0x114 - 0x304
    pub intenset: ReadWrite<u32, Interrupt::Register>,       // 0x304
    pub intenclr: ReadWrite<u32, Interrupt::Register>,       // 0x308
    _reserved4: [u32; 63],                                   // 0x30c - 0x408
    pub hfclkrun: ReadOnly<u32, Status::Register>,           // 0x408
    pub hfclkstat: ReadWrite<u32, HfClkStat::Register>,      // 0x40c
    _reserved5: [u32; 1],                                    // 0x410
    pub lfclkrun: ReadOnly<u32, Control::Register>,          // 0x414
    pub lfclkstat: ReadWrite<u32, LfClkStat::Register>,      // 0x418
    pub lfclksrccopy: ReadOnly<u32, LfClkSrcCopy::Register>, // 0x41c
    _reserved6: [u32; 62],                                   // 0x420 - 0x518
    pub lfclksrc: ReadWrite<u32, LfClkSrc::Register>,        // 0x518
    _reserved7: [u32; 7],                                    // 0x51c - 0x538
    pub ctiv: ReadWrite<u32, Ctiv::Register>,                // 0x538
    _reserved8: [u32; 8],                                    // 0x53c - 0x55c
    pub traceconfig: ReadWrite<u32, TraceConfig::Register>,  // 0x55c
}

register_bitfields! [u32,
    Control [
        ENABLE OFFSET(0) NUMBITS(1)
    ],
    Status [
        READY OFFSET(0) NUMBITS(1)
    ],
    Interrupt [
        HFCLKSTARTED OFFSET(0) NUMBITS(1),
        LFCLKSTARTED OFFSET(1) NUMBITS(1),
        DONE OFFSET(3) NUMBITS(1),
        CTTO OFFSET(4) NUMBITS(1)
    ],
    HfClkStat [
        SRC OFFSET(0) NUMBITS(1) [
            RC = 0,
            XTAL = 1
        ],
        STATE OFFSET(16) NUMBITS(1) [
            RUNNING = 1
        ]
    ],
    LfClkStat [
        SRC OFFSET(0) NUMBITS(2) [
            RC = 0,
            XTAL = 1,
            SYNTH = 2
        ],
        STATE OFFSET(16) NUMBITS(1) [
            RUNNING = 1
        ]
    ],
    LfClkSrcCopy [
        SRC OFFSET(0) NUMBITS(2) [
            RC = 0,
            XTAL = 1,
            SYNTH = 2
        ]
    ],
    LfClkSrc [
        SRC OFFSET(0) NUMBITS(2) [
            RC = 0,
            XTAL = 1,
            SYNTH = 2
        ]
    ],
    Ctiv [
        CTIV OFFSET(0) NUMBITS(7) []
    ],
    TraceConfig [
        TracePortSpeed OFFSET(0) NUMBITS(2) [
            THIRTYTWO = 0,
            SIXTEEN = 1,
            EIGHT = 2,
            FOUR = 3
        ],
        TraceMux OFFSET(16) NUMBITS(2) [
            GPIO = 0,
            SERIAL = 1,
            PARALELL = 2
        ]
    ]
];
