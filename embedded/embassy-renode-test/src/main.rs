#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_nrf::init(Default::default());
    let mut i =0;
    loop {
        i+=1;
        if i == 5 {
            panic!("panic");
        }
        defmt::error!("Blink, {}", i);
        Timer::after(Duration::from_millis(1000)).await;
    }
}
