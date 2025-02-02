#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive, Pin};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let pins = [
        p.P0_31.degrade(), // ok
        p.P0_29.degrade(), // ok
        p.P0_27.degrade(), // ok
        p.P0_26.degrade(), // ok
        p.P0_05.degrade(), // ok
        p.P0_11.degrade(), // ok
        p.P0_03.degrade(), // ok
        p.P0_02.degrade(), // ok
        p.P1_15.degrade(), // ok
        p.P1_13.degrade(), // ok
        p.P1_11.degrade(), // ok
        p.P1_12.degrade(), // ok
        p.P1_04.degrade(),
        p.P1_10.degrade(), // ok
        p.P0_08.degrade(), // ok
        p.P0_13.degrade(),
        p.P0_15.degrade(), // ok
        p.P0_17.degrade(), // ok
        p.P0_21.degrade(), // ok
        p.P0_19.degrade(), // ok
        p.P0_20.degrade(), // ok
        p.P0_22.degrade(), // ok
        p.P0_24.degrade(), // ok
        p.P0_25.degrade(), // ok
        p.P1_02.degrade(), // ok
        p.P0_23.degrade(), // ok
        p.P0_09.degrade(), // ok
        p.P0_10.degrade(), // ok
        p.P1_03.degrade(),
        p.P1_06.degrade(), // ok
    ];
    let mut outputs = pins.map(|pin| Output::new(pin, Level::Low, OutputDrive::Standard));

    loop {
        for output in outputs.iter_mut() {
            output.set_high();
        }
        Timer::after(Duration::from_millis(500)).await;
        for output in outputs.iter_mut() {
            output.set_low();
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}
