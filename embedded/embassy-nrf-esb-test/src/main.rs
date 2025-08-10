#![no_std]
#![no_main]

use defmt::debug;
use defmt_embassy_usb_logger as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf::config::HfclkSource;
use embassy_nrf::gpio::Output;
use embassy_nrf::interrupt::{self, InterruptExt};
use embassy_nrf::peripherals::USBD;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_nrf::usb::Driver;
use embassy_nrf::{bind_interrupts, config::Config, peripherals::RADIO};
use embassy_nrf_esb::ptx::{new_ptx, PtxConfig};
use embassy_nrf_esb::RadioConfig;
use panic_probe as _;

bind_interrupts!(pub struct Irqs {
    RADIO => embassy_nrf_esb::InterruptHandler<RADIO>;
    USBD => embassy_nrf::usb::InterruptHandler<USBD>;
    CLOCK_POWER => embassy_nrf::usb::vbus_detect::InterruptHandler;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = Config::default();
    let p = embassy_nrf::init(config);

    interrupt::RADIO.set_priority(interrupt::Priority::P1);

    spawner
        .spawn(logger(Driver::new(
            p.USBD,
            Irqs,
            HardwareVbusDetect::new(Irqs),
        )))
        .unwrap();

    defmt::info!("start");

    let (mut task, ptx) =
        new_ptx::<_, 255>(p.RADIO, Irqs, RadioConfig::default(), PtxConfig::default());
    join(task.run(), async move {
        loop {
            if let Err(e) = ptx.send(0, &[0, 1, 2, 3], true).await {
                debug!("{:?}", e);
            }
            embassy_time::Timer::after_millis(1000).await;
        }
    })
    .await;

    // let mut ptx = embassy_nrf_esb::ptx::PtxRadio::<'_, _, 64>::new(
    //     p.RADIO,
    //     Irqs,
    //     embassy_nrf_esb::RadioConfig::default(),
    //     embassy_nrf_esb::ptx::PtxConfig::default(),
    // )
    // .unwrap();
    // let mut i = 0;
    // loop {
    //     let res = ptx.send(0, &[0, 0, i], false).await;
    //     defmt::info!("send {:?}", res);
    //     embassy_time::Timer::after_secs(1).await;
    //     i += 1;
    // }
}

#[embassy_executor::task]
async fn logger(driver: Driver<'static, USBD, HardwareVbusDetect>) {
    defmt_embassy_usb_logger::logger_task(driver, 0xCA, 0xFE).await;
}
