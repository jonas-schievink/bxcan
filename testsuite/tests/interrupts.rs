#![no_std]
#![no_main]

#[defmt_test::tests]
mod tests {
    use core::{
        cell::RefCell,
        sync::atomic::{AtomicBool, Ordering},
    };

    use bxcan::{
        filter::{ListEntry32, Mask16, Mask32},
        ExtendedId, StandardId,
    };
    use bxcan::{Frame, Interrupt};

    use cortex_m::interrupt::Mutex;
    use irq::handler;
    use nb::block;
    use testsuite::{interrupt, State};

    #[init]
    fn init() -> State {
        State::init()
    }

    #[test]
    fn tx_interrupt(state: &mut State) {
        state
            .can1
            .modify_filters()
            .clear()
            .enable_bank(0, Mask32::accept_all());

        state.can1.enable_interrupt(Interrupt::TransmitMailboxEmpty);

        let state = Mutex::new(RefCell::new(state));
        let tx_fired = AtomicBool::new(false);
        handler!(
            can1_tx = || {
                defmt::debug!("CAN1 TX interrupt");
                cortex_m::interrupt::free(|cs| {
                    state.borrow(cs).borrow_mut().can1.clear_tx_interrupt();
                });
                tx_fired.store(true, Ordering::Relaxed);
            }
        );
        irq::scope(|scope| {
            scope.register(interrupt::CAN1_TX, can1_tx);

            defmt::assert!(!tx_fired.load(Ordering::Relaxed));
            let frame = Frame::new_data(StandardId::new(0).unwrap(), []);
            cortex_m::interrupt::free(|cs| {
                defmt::assert!(state.borrow(cs).borrow_mut().roundtrip_frame(&frame));
            });
            defmt::assert!(tx_fired.load(Ordering::Relaxed));
        });
    }
}
