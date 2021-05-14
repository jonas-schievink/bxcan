//! Interrupt types.

use core::ops;

use defmt::Format;

#[allow(unused_imports)] // for intra-doc links only
use crate::{Can, Rx};

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
    /// Fires the **TX** interrupt when one of the transmit mailboxes returns to empty state.
    ///
    /// This usually happens because its message was either transmitted successfully, or
    /// transmission was aborted successfully.
    ///
    /// The interrupt handler must clear the interrupt condition by calling
    /// [`Can::clear_request_completed_flag`] or [`Can::clear_tx_interrupt`].
    TransmitMailboxEmpty = 1 << 0,

    /// Fires the **RX FIFO 0** interrupt when FIFO 0 holds a message.
    ///
    /// The interrupt handler must clear the interrupt condition by receiving all messages from the
    /// FIFO by calling [`Can::receive`] or [`Rx::receive`].
    Fifo0MessagePending = 1 << 1,

    /// Fires the **RX FIFO 0** interrupt when FIFO 0 holds 3 incoming messages.
    ///
    /// The interrupt handler must clear the interrupt condition by receiving at least one message
    /// from the FIFO (making it no longer "full"). This can be done by calling [`Can::receive`] or
    /// [`Rx::receive`].
    Fifo0Full = 1 << 2,

    /// Fires the **RX FIFO 0** interrupt when FIFO 0 drops an incoming message.
    ///
    /// The interrupt handler must clear the interrupt condition by calling [`Can::receive`] or
    /// [`Rx::receive`] (which will return an error).
    Fifo0Overrun = 1 << 3,

    /// Fires the **RX FIFO 1** interrupt when FIFO 1 holds a message.
    ///
    /// Behavior is otherwise identical to [`Self::Fifo0MessagePending`].
    Fifo1MessagePending = 1 << 4,

    /// Fires the **RX FIFO 1** interrupt when FIFO 1 holds 3 incoming messages.
    ///
    /// Behavior is otherwise identical to [`Self::Fifo0Full`].
    Fifo1Full = 1 << 5,

    /// Fires the **RX FIFO 1** interrupt when FIFO 1 drops an incoming message.
    ///
    /// Behavior is otherwise identical to [`Self::Fifo0Overrun`].
    Fifo1Overrun = 1 << 6,

    Error = 1 << 15,
    Wakeup = 1 << 16,
    Sleep = 1 << 17,
}

bitflags::bitflags! {
    /// A set of bxCAN interrupts.
    pub struct Interrupts: u32 {
        const TRANSMIT_MAILBOX_EMPTY = 1 << 0;
        const FIFO0_MESSAGE_PENDING = 1 << 1;
        const FIFO0_FULL = 1 << 2;
        const FIFO0_OVERRUN = 1 << 3;
        const FIFO1_MESSAGE_PENDING = 1 << 4;
        const FIFO1_FULL = 1 << 5;
        const FIFO1_OVERRUN = 1 << 6;
        const ERROR = 1 << 15;
        const WAKEUP = 1 << 16;
        const SLEEP = 1 << 17;
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
