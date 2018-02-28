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

use vcell::VolatileCell;

const CLOCK_BASE: usize = 0x40000000;

pub static mut CLOCK: Clock = Clock::new();

/// Interrupts
pub enum InterruptField {
    HFCLKSTARTED = (1 << 0),
    LFCLKSTARTED = (1 << 1),
    DONE = (1 << 3),
    CTTO = (1 << 4),
}

#[allow(dead_code)]
enum ClockRunning {
    NORUN = 0,
    RUN = (1 << 16),
}

pub enum LowClockSource {
    RC = 0,
    XTAL = 1,
    SYNTH = 2,
}

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

    pub unsafe fn interrupt_enable(&self, interrupt: InterruptField) {
        let regs = &*self.registers;
        regs.intenset.set(interrupt as u32);
    }

    pub unsafe fn interrupt_disable(&self, interrupt: InterruptField) {
        let regs = &*self.registers;
        regs.intenclr.set(interrupt as u32);
    }

    pub unsafe fn high_start(&self) {
        let regs = &*self.registers;
        regs.tasks_hfclkstart.set(1);
    }

    pub unsafe fn high_stop(&self) {
        let regs = &*self.registers;
        regs.tasks_hfclkstop.set(1);
    }

    pub unsafe fn high_started(&self) -> bool {
        let regs = &*self.registers;
        regs.events_hfclkstarted.get() == 1
    }

    pub unsafe fn high_set_source(&self, source: HighClockSource) {
        let regs = &*self.registers;
        regs.hfclkstat.set(source as u32);
    }

    pub unsafe fn high_running(&self) -> bool {
        let regs = &*self.registers;
        (regs.hfclkstat.get() & ClockRunning::RUN as u32) == ClockRunning::RUN as u32
    }

    pub unsafe fn low_start(&self) {
        let regs = &*self.registers;
        regs.tasks_lfclkstart.set(1);
    }

    pub unsafe fn low_stop(&self) {
        let regs = &*self.registers;
        regs.tasks_lfclkstop.set(1);
    }

    pub unsafe fn low_started(&self) -> bool {
        let regs = &*self.registers;
        regs.events_lfclkstarted.get() == 1
    }

    pub unsafe fn low_running(&self) -> bool {
        let regs = &*self.registers;
        (regs.lfclkstat.get() & ClockRunning::RUN as u32) == ClockRunning::RUN as u32
    }

    pub unsafe fn low_set_source(&self, src: LowClockSource) {
        let regs = &*self.registers;
        regs.lfclksrc.set(src as u32);
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[repr(C)]
struct ClockRegisters {
    pub tasks_hfclkstart: VolatileCell<u32>,    // 0x000
    pub tasks_hfclkstop: VolatileCell<u32>,     // 0x004
    pub tasks_lfclkstart: VolatileCell<u32>,    // 0x008    
    pub tasks_lfclkstop: VolatileCell<u32>,     // 0x00c
    pub tasks_cal: VolatileCell<u32>,           // 0x010
    pub tasks_ctstart: VolatileCell<u32>,       // 0x014
    pub tasks_ctstop: VolatileCell<u32>,        // 0x018
    _reserved1: [VolatileCell<u32>; 57],        // 0x018 - 0x100
    pub events_hfclkstarted: VolatileCell<u32>, // 0x100
    pub events_lfclkstarted: VolatileCell<u32>, // 0x104
    _reserverd2: VolatileCell<u32>,             // 0x108    
    pub events_done: VolatileCell<u32>,         // 0x10c
    pub events_ctto: VolatileCell<u32>,         // 0x110
    _reserved3: [VolatileCell<u32>; 124],       // 0x114 - 0x304
    pub intenset: VolatileCell<u32>,            // 0x304 
    pub intenclr: VolatileCell<u32>,            // 0x308
    _reserved4: [VolatileCell<u32>; 63],        // 0x30c - 0x408
    pub hfclkrun: VolatileCell<u32>,            // 0x408
    pub hfclkstat: VolatileCell<u32>,           // 0x40c
    _reserved5: [VolatileCell<u32>; 1],         // 0x410
    pub lfclkrun: VolatileCell<u32>,            // 0x414
    pub lfclkstat: VolatileCell<u32>,           // 0x418
    pub lfclksrccopy: VolatileCell<u32>,        // 0x41c
    _reserved6: [VolatileCell<u32>; 62],        // 0x420 - 0x518
    pub lfclksrc: VolatileCell<u32>,            // 0x518
    _reserved7: [VolatileCell<u32>; 7],         // 0x51c - 0x538
    pub ctiv: VolatileCell<u32>,                // 0x538
    _reserved8: [VolatileCell<u32>; 8],         // 0x53c - 0x55c
    pub traceconfig: VolatileCell<u32>,         // 0x55c
}
