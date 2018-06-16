//! UART

use board;
use tock_regs::regs::{ReadOnly, ReadWrite, WriteOnly};

const UART_BASE: u32 = 0x4000_2000;

/// Uart Singleton
pub static UART: Uart = Uart::new();

/// Uart
pub struct Uart {
    registers: *const UartTeRegisters,
}

impl Uart {
    /// Constructor
    pub const fn new() -> Self {
        Uart {
            registers: UART_BASE as *const UartTeRegisters,
        }
    }

    /// Initialize UART
    /// Configures pins to be used by the UART and the baudrate
    pub unsafe fn initialize(&self, baudrate: u32) {
        self.set_pins();
        self.set_baudrate(baudrate);
    }

    unsafe fn set_pins(&self) {
        let regs = &*self.registers;
        regs.pseltxd.write(Psel::PIN.val(board::UART_TXD));
        regs.pselrxd.write(Psel::PIN.val(board::UART_RXD));
        regs.pselcts.write(Psel::PIN.val(board::UART_CTS));
        regs.pselrts.write(Psel::PIN.val(board::UART_RTS));
    }

    unsafe fn set_baudrate(&self, baudrate: u32) {
        let regs = &*self.registers;
        match baudrate {
            1200 => regs.baudrate.set(0x0004_F000),
            2400 => regs.baudrate.set(0x0009_D000),
            4800 => regs.baudrate.set(0x0013_B000),
            9600 => regs.baudrate.set(0x0027_5000),
            14400 => regs.baudrate.set(0x003A_F000),
            19200 => regs.baudrate.set(0x004E_A000),
            28800 => regs.baudrate.set(0x0075_C000),
            38400 => regs.baudrate.set(0x009D_0000),
            57600 => regs.baudrate.set(0x00EB_0000),
            76800 => regs.baudrate.set(0x013A_9000),
            115200 => regs.baudrate.set(0x01D6_0000),
            230400 => regs.baudrate.set(0x03B0_0000),
            250000 => regs.baudrate.set(0x0400_0000),
            460800 => regs.baudrate.set(0x0740_0000),
            921600 => regs.baudrate.set(0x0F00_0000),
            1000000 => regs.baudrate.set(0x1000_0000),
            _ => regs.baudrate.set(0x01D6_0000), //setting default to 115200
        }
    }

    #[allow(unused)]
    /// Interrupts are not enabled yet
    unsafe fn enable_interrupts(&self) {
        let regs = &*self.registers;
        regs.intenset
            .write(Interrupt::ENDTX::SET + Interrupt::ERROR::SET + Interrupt::TXSTARTED::SET);
    }

    /// Transmit
    pub unsafe fn transmit(&self, buffer: &'static [u8]) {
        let regs = &*self.registers;

        if buffer.is_empty() {
            return;
        }

        let ptr = buffer.as_ptr();
        regs.txd_ptr.set(ptr as u32);
        regs.txd_maxcnt.set(buffer.len() as u32);
        regs.enable.write(Enable::ENABLE::ENABLED);
        regs.task_stoptx.write(Task::ENABLE::SET);
        regs.task_starttx.write(Task::ENABLE::SET);

        // busy-wait
        while regs.event_txstarted.matches_any(Event::READY::CLEAR) {}

        // busy-wait
        while regs.event_endtx.matches_any(Event::READY::CLEAR) {}
    }
}

unsafe impl Send for Uart {}
unsafe impl Sync for Uart {}

#[repr(C)]
struct UartTeRegisters {
    pub task_startrx: WriteOnly<u32, Task::Register>, // 0x000
    pub task_stoprx: WriteOnly<u32, Task::Register>,  // 0x004
    pub task_starttx: WriteOnly<u32, Task::Register>, // 0x008
    pub task_stoptx: WriteOnly<u32, Task::Register>,  // 0x00c
    _reserved1: [u32; 7],                             // 0x010-0x02c
    pub task_flush_rx: WriteOnly<u32, Task::Register>, // 0x02c
    _reserved2: [u32; 52],                            // 0x030-0x100
    pub event_cts: ReadOnly<u32, Event::Register>,    // 0x100-0x104
    pub event_ncts: ReadOnly<u32, Event::Register>,   // 0x104-0x108
    _reserved3: [u32; 2],                             // 0x108-0x110
    pub event_endrx: ReadOnly<u32, Event::Register>,  // 0x110-0x114
    _reserved4: [u32; 3],                             // 0x114-0x120
    pub event_endtx: ReadOnly<u32, Event::Register>,  // 0x120-0x124
    pub event_error: ReadOnly<u32, Event::Register>,  // 0x124-0x128
    _reserved6: [u32; 7],                             // 0x128-0x144
    pub event_rxto: ReadOnly<u32, Event::Register>,   // 0x144-0x148
    _reserved7: [u32; 1],                             // 0x148-0x14C
    pub event_rxstarted: ReadOnly<u32, Event::Register>, // 0x14C-0x150
    pub event_txstarted: ReadOnly<u32, Event::Register>, // 0x150-0x154
    _reserved8: [u32; 1],                             // 0x154-0x158
    pub event_txstopped: ReadOnly<u32, Event::Register>, // 0x158-0x15c
    _reserved9: [u32; 41],                            // 0x15c-0x200
    pub shorts: ReadWrite<u32, Shorts::Register>,     // 0x200-0x204
    _reserved10: [u32; 64],                           // 0x204-0x304
    pub intenset: ReadWrite<u32, Interrupt::Register>, // 0x304-0x308
    pub intenclr: ReadWrite<u32, Interrupt::Register>, // 0x308-0x30C
    _reserved11: [u32; 93],                           // 0x30C-0x480
    pub errorsrc: ReadWrite<u32, ErrorSrc::Register>, // 0x480-0x484
    _reserved12: [u32; 31],                           // 0x484-0x500
    pub enable: ReadWrite<u32, Enable::Register>,     // 0x500-0x504
    _reserved13: [u32; 1],                            // 0x504-0x508
    pub pselrts: ReadWrite<u32, Psel::Register>,      // 0x508-0x50c
    pub pseltxd: ReadWrite<u32, Psel::Register>,      // 0x50c-0x510
    pub pselcts: ReadWrite<u32, Psel::Register>,      // 0x510-0x514
    pub pselrxd: ReadWrite<u32, Psel::Register>,      // 0x514-0x518
    _reserved14: [u32; 3],                            // 0x518-0x524
    pub baudrate: ReadWrite<u32, Baudrate::Register>, // 0x524-0x528
    _reserved15: [u32; 3],                            // 0x528-0x534
    pub rxd_ptr: ReadWrite<u32, Pointer::Register>,   // 0x534-0x538
    pub rxd_maxcnt: ReadWrite<u32, Counter::Register>, // 0x538-0x53c
    pub rxd_amount: ReadOnly<u32, Counter::Register>, // 0x53c-0x540
    _reserved16: [u32; 1],                            // 0x540-0x544
    pub txd_ptr: ReadWrite<u32, Pointer::Register>,   // 0x544-0x548
    pub txd_maxcnt: ReadWrite<u32, Counter::Register>, // 0x548-0x54c
    pub txd_amount: ReadOnly<u32, Counter::Register>, // 0x54c-0x550
    _reserved17: [u32; 7],                            // 0x550-0x56C
    pub config: ReadWrite<u32, Config::Register>,     // 0x56C-0x570
}

#[cfg_attr(rustfmt, rustfmt_skip)]
register_bitfields! [u32,
    /// Start task
    Task [
        ENABLE OFFSET(0) NUMBITS(1)
    ],

    /// Read event
    Event [
        READY OFFSET(0) NUMBITS(1)
    ],
    
    /// Shortcuts
    Shorts [
        /// Shortcut between ENDRX and STARTRX
        ENDRX_STARTRX OFFSET(5) NUMBITS(1),
        /// Shortcut between ENDRX and STOPRX
        ENDRX_STOPRX OFFSET(6) NUMBITS(1)
    ],

    /// UART Interrups
    Interrupt [
        CTS OFFSET(0) NUMBITS(1),
        NCTS OFFSET(1) NUMBITS(1),
        ENDRX OFFSET(4) NUMBITS(1),
        ENDTX OFFSET(8) NUMBITS(1),
        ERROR OFFSET(9) NUMBITS(1),
        RXTO OFFSET(17) NUMBITS(1),
        RXSTARTED OFFSET(19) NUMBITS(1),
        TXSTARTED OFFSET(20) NUMBITS(1),
        TXSTOPPED OFFSET(22) NUMBITS(1)
    ],
    
    /// UART Errors
    ErrorSrc [
        OVERRUN OFFSET(0) NUMBITS(1),
        PARITY OFFSET(1) NUMBITS(1),
        FRAMING OFFSET(2) NUMBITS(1),
        BREAK OFFSET(3) NUMBITS(1)
    ],
    
    /// Enable UART
    Enable [
        ENABLE OFFSET(0) NUMBITS(4) [
           ENABLED = 8,
           DISABLED = 0
        ]
    ],
    
    /// Pin select
    Psel [
        /// Pin number
        PIN OFFSET(0) NUMBITS(5),
        /// Connect/Disconnect
        CONNECT OFFSET(31) NUMBITS(1)
    ],
    
    /// Baudrate
    Baudrate [
        BAUDRAUTE OFFSET(0) NUMBITS(32)
    ],
    
    /// DMA pointer
    Pointer [
        POINTER OFFSET(0) NUMBITS(32)
    ],
    
    /// Counter value
    Counter [
        COUNTER OFFSET(0) NUMBITS(8)
    ],
    
    /// Configuration of parity and flow control
    Config [
        HWFC OFFSET(0) NUMBITS(1),
        PARITY OFFSET(1) NUMBITS(3)
    ]
];
