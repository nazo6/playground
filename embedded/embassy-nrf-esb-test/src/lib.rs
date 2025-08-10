#![no_std]

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    config::Config,
    interrupt::{self, InterruptExt as _},
    peripherals::{RADIO, USBD},
    usb::{vbus_detect::HardwareVbusDetect, Driver},
    Peripherals,
};

bind_interrupts!(pub struct Irqs {
    RADIO => embassy_nrf_esb::InterruptHandler<RADIO>;
    USBD => embassy_nrf::usb::InterruptHandler<USBD>;
    CLOCK_POWER => embassy_nrf::usb::vbus_detect::InterruptHandler;
});

pub fn init(spawner: Spawner, vid: u16, pid: u16) -> Peripherals {
    let config = Config::default();
    let p = embassy_nrf::init(config);

    interrupt::RADIO.set_priority(interrupt::Priority::P1);

    spawner
        .spawn(logger(
            Driver::new(
                unsafe { p.USBD.clone_unchecked() },
                Irqs,
                HardwareVbusDetect::new(Irqs),
            ),
            vid,
            pid,
        ))
        .unwrap();
    p
}

#[embassy_executor::task]
async fn logger(driver: Driver<'static, USBD, HardwareVbusDetect>, vid: u16, pid: u16) {
    defmt_embassy_usb_logger::logger_task(driver, vid, pid).await;
}
