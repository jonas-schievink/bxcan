//! Interrupt types.

use core::ops;

use defmt::Format;

/// bxCAN interrupt sources.
///
/// These can be individually enabled and disabled in the bxCAN peripheral. Note that the bxCAN
/// peripheral only exposes 4 interrupts to the microcontroller:
///
/// * TX
/// * RX FIFO 1
/// * RX FIFO 2
/// * SCE (Status Change Error)
///
/// This means that some of the interrupts listed here will result in the same interrupt handler
/// being invoked.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Format)]
#[non_exhaustive]
pub enum Interrupt {
    Sleep = 1 << 17,
    Wakeup = 1 << 16,
    Error = 1 << 15,
    Fifo1Overrun = 1 << 6,
    Fifo1Full = 1 << 5,
    Fifo1MessagePending = 1 << 4,
    Fifo0Overrun = 1 << 3,
    Fifo0Full = 1 << 2,
    Fifo0MessagePending = 1 << 1,
    /// Fires the **TX** interrupt when one of the transmit mailboxes returns to empty state.
    /// 
    /// This usually happens because its message was either transmitted successfully, or
    /// transmission was aborted successfully.
    ///
    /// The interrupt handler must acknowledge the interrupt (TODO how?)
    TransmitMailboxEmpty = 1 << 0,
}

bitflags::bitflags! {
    /// A set of bxCAN interrupts.
    pub struct Interrupts: u32 {
        const SLEEP = 1 << 17;
        const WAKEUP = 1 << 16;
        const ERROR = 1 << 15;
        const FIFO1_OVERRUN = 1 << 6;
        const FIFO1_FULL = 1 << 5;
        const FIFO1_MESSAGE_PENDING = 1 << 4;
        const FIFO0_OVERRUN = 1 << 3;
        const FIFO0_FULL = 1 << 2;
        const FIFO0_MESSAGE_PENDING = 1 << 1;
        const TRANSMIT_MAILBOX_EMPTY = 1 << 0;
    }
}

impl From<Interrupt> for Interrupts {
    #[inline]
    fn from(i: Interrupt) -> Self {
        Self::from_bits_truncate(i as u32)
    }
}

/// Adds an interrupt to the interrupt set.
impl ops::BitOrAssign<Interrupt> for Interrupts {
    #[inline]
    fn bitor_assign(&mut self, rhs: Interrupt) {
        *self |= Self::from(rhs);
    }
}
