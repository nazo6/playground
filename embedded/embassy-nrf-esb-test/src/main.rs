#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use defmt::debug;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf::config::HfclkSource;
use embassy_nrf::gpio::Output;
use embassy_nrf::interrupt::{self, InterruptExt};
use embassy_nrf::{bind_interrupts, config::Config, peripherals::RADIO};
use embassy_nrf_esb::ptx::{new_ptx, PtxConfig};
use embassy_nrf_esb::RadioConfig;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(pub struct Irqs {
    RADIO => embassy_nrf_esb::InterruptHandler<RADIO>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    config.hfclk_source = HfclkSource::ExternalXtal;
    let p = embassy_nrf::init(config);

    interrupt::RADIO.set_priority(interrupt::Priority::P1);

    let mut led = Output::new(
        p.P1_03,
        embassy_nrf::gpio::Level::Low,
        embassy_nrf::gpio::OutputDrive::Standard,
    );

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
