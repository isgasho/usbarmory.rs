//! Turns the blue LED on and the white LED off

#![no_main]
#![no_std]

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::led;

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    if let Some(blue) = led::Blue::take() {
        blue.on();
    }

    if let Some(white) = led::White::take() {
        white.off();
    }

    // wait 5 seconds
    usbarmory::delay(5 * usbarmory::CPU_FREQUENCY);

    // then reset the board to return to the u-boot console
    usbarmory::reset()
}
