//! LEDs

use core::{
    marker::PhantomData,
    sync::atomic::{AtomicBool, Ordering},
};

use rac::gpio;

// FIXME the two LEDs should be packed in a struct and it should not be possible
// to move either out of that struct (rationale: concurrently acting on the LEDs
// can lead to loss of GPIO4_DR data)

const BLUE: u32 = 1 << 22;
const WHITE: u32 = 1 << 21;

/// Blue LED
pub struct Blue {
    _not_sync: PhantomData<*mut ()>,
}

unsafe impl Send for Blue {}

static BLUE_TAKEN: AtomicBool = AtomicBool::new(false);

impl Blue {
    /// Gets an exclusive handle to the `Blue` singleton
    pub fn take() -> Option<Self> {
        if BLUE_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            Some(Blue {
                _not_sync: PhantomData,
            })
        } else {
            None
        }
    }

    /// Release the exclusive handle so any other context can take it
    pub fn release(self) {
        BLUE_TAKEN.store(false, Ordering::Release)
    }

    /// Turns the LED off
    pub fn off(&self) {
        unsafe {
            let old = gpio::GPIO4_DR.read_volatile();
            gpio::GPIO4_DR.write_volatile(old | BLUE);
        }
    }

    /// Turns the LED on
    pub fn on(&self) {
        unsafe {
            let old = gpio::GPIO4_DR.read_volatile();
            gpio::GPIO4_DR.write_volatile(old & !BLUE);
        }
    }

    /// Toggles the LED
    pub fn toggle(&self) {
        unsafe {
            let old = gpio::GPIO4_DR.read_volatile();
            if old & BLUE == 0 {
                gpio::GPIO4_DR.write_volatile(old | BLUE);
            } else {
                gpio::GPIO4_DR.write_volatile(old & !BLUE);
            }
        }
    }
}

/// White LED
pub struct White {
    _not_sync: PhantomData<*mut ()>,
}

unsafe impl Send for White {}

static WHITE_TAKEN: AtomicBool = AtomicBool::new(false);

impl White {
    /// Gets an exclusive handle to the `White` singleton
    pub fn take() -> Option<Self> {
        if WHITE_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            Some(White {
                _not_sync: PhantomData,
            })
        } else {
            None
        }
    }

    /// Release the exclusive handle so any other context can take it
    pub fn release(self) {
        WHITE_TAKEN.store(false, Ordering::Release)
    }

    /// Turns the LED off
    pub fn off(&self) {
        unsafe {
            let old = gpio::GPIO4_DR.read_volatile();
            gpio::GPIO4_DR.write_volatile(old | WHITE);
        }
    }

    /// Turns the LED on
    pub fn on(&self) {
        unsafe {
            let old = gpio::GPIO4_DR.read_volatile();
            gpio::GPIO4_DR.write_volatile(old & !WHITE);
        }
    }
    /// Toggles the LED
    pub fn toggle(&self) {
        unsafe {
            let old = gpio::GPIO4_DR.read_volatile();
            if old & WHITE == 0 {
                gpio::GPIO4_DR.write_volatile(old | WHITE);
            } else {
                gpio::GPIO4_DR.write_volatile(old & !WHITE);
            }
        }
    }
}
