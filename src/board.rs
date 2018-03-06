//! Board definitions for nrf52dk

// The nRF52 DK LEDs (see back of board)
const LED1_PIN: u32 = 17;
const LED2_PIN: u32 = 18;
const LED3_PIN: u32 = 19;
const LED4_PIN: u32 = 20;

// The nRF52 DK buttons (see back of board)
const BUTTON1_PIN: u32 = 13;
const BUTTON2_PIN: u32 = 14;
const BUTTON3_PIN: u32 = 15;
const BUTTON4_PIN: u32 = 16;
const BUTTON_RESET_PIN: u32 = 21;

/// NRF52-DKs LEDs
pub const LEDS: [u32; 4] = [LED1_PIN, LED2_PIN, LED3_PIN, LED4_PIN];
/// NRF52-DKs Buttons
pub const BUTTONS: [u32; 5] =
    [BUTTON1_PIN, BUTTON2_PIN, BUTTON3_PIN, BUTTON4_PIN, BUTTON_RESET_PIN];

/// NRF52-DK UART RTS
pub const UART_RTS: u32 = 5;
/// NRF52-DK UART TXD
pub const UART_TXD: u32 = 6;
/// NRF52-DK UART CTS
pub const UART_CTS: u32 = 7;
/// NRF52-DK UART RXD
pub const UART_RXD: u32 = 8;
