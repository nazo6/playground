#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_nrf::config::HfclkSource;
use embassy_nrf::{bind_interrupts, config::Config, peripherals::RADIO};
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

    defmt::info!("start");
    let mut prx = PrxRadio::<'_, _, 64>::new(p.RADIO, Irqs, RadioConfig::default()).unwrap();
    loop {
        let mut buf = [0; 64];
        let res = prx.recv(&mut buf, 0xFF).await;
        defmt::info!("{:?},{:?}", buf, res);
    }
}
