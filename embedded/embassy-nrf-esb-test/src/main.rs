#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_nrf::config::HfclkSource;
use embassy_nrf::gpio::Output;
use embassy_nrf::{bind_interrupts, config::Config, peripherals::RADIO};
use embassy_nrf_esb::ptx::PtxRadio;
use embassy_nrf_esb::{prx::PrxRadio, RadioConfig};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(pub struct Irqs {
    RADIO => embassy_nrf_esb::InterruptHandler<RADIO>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    config.hfclk_source = HfclkSource::ExternalXtal;
    let p = embassy_nrf::init(config);

    let mut led = Output::new(
        p.P1_03,
        embassy_nrf::gpio::Level::Low,
        embassy_nrf::gpio::OutputDrive::Standard,
    );

    defmt::info!("start");
    let mut prx = PrxRadio::<'_, _, 64>::new(p.RADIO, Irqs, RadioConfig::default()).unwrap();
    loop {
        let mut buf = [0; 32];
        if let Ok(s) = prx.recv(&mut buf, 0xFF).await {
            defmt::info!("recv {:?}", buf[..s]);
        }
        // led.toggle();
    }

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
