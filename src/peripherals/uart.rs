//! UART

use vcell::VolatileCell;
use board;

const UART_BASE: u32 = 0x40002000;

/// Uart Singleton
pub static mut UART: Uart = Uart::new();

/// Uart
pub struct Uart {
    registers: *const UartTeRegisters, 
    // buffer: [u8; 64],
}

impl Uart {
    /// Constructor
    pub const fn new() -> Self {
        Uart {
            registers: UART_BASE as *const UartTeRegisters, 
            // buffer: [0; 64],
        }
    }

    /// Initialize UART
    pub unsafe fn initialize(&self) {
        let regs = &*self.registers;

        regs.pseltxd.set(board::UART_TXD);
        regs.pselrxd.set(board::UART_RXD);
        regs.pselcts.set(board::UART_CTS);
        regs.pselrts.set(board::UART_RTS);

        regs.baudrate.set(0x01D60000);
    }

    /// Transmit
    pub unsafe fn transmit(&self, buffer: &'static [u8]) {
        let regs = &*self.registers;

        // if buffer.len() == 0 {
        //     return;
        // }

        let ptr = buffer.as_ptr();
        // let ptr2 = &BUFFER;
        
        regs.txd_ptr.set(ptr as u32);
        // regs.txd_maxcnt.set(buffer.len() as u32);
        regs.event_endtx.set(0);
        regs.task_starttx.set(1);

        // busy-wait
        for _ in 0..10000000 {}
    }
}

#[repr(C)]
struct UartTeRegisters {
    pub task_startrx: VolatileCell<u32>, // 0x000-0x004
    pub task_stoprx: VolatileCell<u32>, // 0x004-0x008
    pub task_starttx: VolatileCell<u32>, // 0x008-0x00c
    pub task_stoptx: VolatileCell<u32>, // 0x00c-0x010
    _reserved1: [u32; 7], // 0x010-0x02c
    pub task_flush_rx: VolatileCell<u32>, // 0x02c-0x030
    _reserved2: [u32; 52], // 0x030-0x100
    pub event_cts: VolatileCell<u32>, // 0x100-0x104
    pub event_ncts: VolatileCell<u32>, // 0x104-0x108
    _reserved3: [u32; 2], // 0x108-0x110
    pub event_endrx: VolatileCell<u32>, // 0x110-0x114
    _reserved4: [u32; 3], // 0x114-0x120
    pub event_endtx: VolatileCell<u32>, // 0x120-0x124
    pub event_error: VolatileCell<u32>, // 0x124-0x128
    _reserved6: [u32; 7], // 0x128-0x144
    pub event_rxto: VolatileCell<u32>, // 0x144-0x148
    _reserved7: [u32; 1], // 0x148-0x14C
    pub event_rxstarted: VolatileCell<u32>, // 0x14C-0x150
    pub event_txstarted: VolatileCell<u32>, // 0x150-0x154
    _reserved8: [u32; 1], // 0x154-0x158
    pub event_txstopped: VolatileCell<u32>, // 0x158-0x15c
    _reserved9: [u32; 41], // 0x15c-0x200
    pub shorts: VolatileCell<u32>, // 0x200-0x204
    _reserved10: [u32; 64], // 0x204-0x304
    pub intenset: VolatileCell<u32>, // 0x304-0x308
    pub intenclr: VolatileCell<u32>, // 0x308-0x30C
    _reserved11: [u32; 93], // 0x30C-0x480
    pub errorsrc: VolatileCell<u32>, // 0x480-0x484
    _reserved12: [u32; 31], // 0x484-0x500
    pub enable: VolatileCell<u32>, // 0x500-0x504
    _reserved13: [u32; 1], // 0x504-0x508
    pub pselrts: VolatileCell<u32>, // 0x508-0x50c
    pub pseltxd: VolatileCell<u32>, // 0x50c-0x510
    pub pselcts: VolatileCell<u32>, // 0x510-0x514
    pub pselrxd: VolatileCell<u32>, // 0x514-0x518
    _reserved14: [u32; 3], // 0x518-0x524
    pub baudrate: VolatileCell<u32>, // 0x524-0x528
    _reserved15: [u32; 3], // 0x528-0x534
    pub rxd_ptr: VolatileCell<u32>, // 0x534-0x538
    pub rxd_maxcnt: VolatileCell<u32>, // 0x538-0x53c
    pub rxd_amount: VolatileCell<u32>, // 0x53c-0x540
    _reserved16: [u32; 1], // 0x540-0x544
    pub txd_ptr: VolatileCell<u32>, // 0x544-0x548
    pub txd_maxcnt: VolatileCell<u32>, // 0x548-0x54c
    pub txd_amount: VolatileCell<u32>, // 0x54c-0x550
    _reserved17: [u32; 7], // 0x550-0x56C
    pub config: VolatileCell<u32>, // 0x56C-0x570
}
