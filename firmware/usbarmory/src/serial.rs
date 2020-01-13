//! Serial interface

use core::{
    fmt,
    marker::PhantomData,
    sync::atomic::{AtomicBool, Ordering},
};

use rac::uart;

static TAKEN: AtomicBool = AtomicBool::new(false);

/// Handle to the serial interface
pub struct Serial {
    _not_sync: PhantomData<*mut ()>,
}

unsafe impl Send for Serial {}

impl Serial {
    // NOTE u-boot already initialized this
    /// Gets an exclusive handle to the `Serial` singleton
    pub fn take() -> Option<Self> {
        if TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            Some(Serial {
                _not_sync: PhantomData,
            })
        } else {
            None
        }
    }

    /// Gets a handle to the `Serial` singleton even if it's currently owned by
    /// some other context
    ///
    /// WARNING: using `borrow_unchecked` + `write_all` in concurrent contexts
    /// can result in data loss
    // NOTE(safety) at the moment this is not unsound because `Serial` only uses
    // MMIO registers which have strongly-ordered, single-instruction load and
    // store (but not Read-Modify-Write) operations. If the abstraction at some
    // point starts *internally* using normal memory (e.g. a buffer in RAM) then
    // this operation would need to become `unsafe`.
    pub fn borrow_unchecked<R>(f: impl FnOnce(&Self) -> R) -> R {
        let serial = Serial {
            _not_sync: PhantomData,
        };
        f(&serial)
    }

    /// Release the exclusive handle so any other context can take it
    pub fn release(self) {
        TAKEN.store(false, Ordering::Release);
    }

    /// Blocks until all data has been transmitted
    pub fn flush(&self) {
        unsafe { while uart::UART2_USR2.read_volatile() & uart::UART_USR2_TXDC == 0 {} }
    }

    /// [Blocking] Sends the given `bytes` through the serial interface
    pub fn write_all(&self, bytes: &[u8]) {
        for byte in bytes {
            unsafe {
                // if the FIFO buffer is full wait until we can write the next byte
                while uart::UART2_USR1.read_volatile() & uart::UART_USR1_TRDY == 0 {}

                uart::UART2_UTXD.write_volatile(*byte as u32);
            }
        }
    }
}

impl fmt::Write for &'_ Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes());
        Ok(())
    }
}
