#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use core::panic::PanicInfo;

use bitvec::prelude::*;
use embassy_executor::Spawner;
use embassy_nrf::{
    gpio::OutputDrive,
    pwm::{Config, Prescaler, SequenceLoad, SequencePwm, SingleSequenceMode, SingleSequencer},
};
use embassy_time::Timer;
use smart_leds::RGB8;

use defmt_rtt as _;
use nrf_softdevice as _;

const T1H: u16 = 0x8000 | 13; // Duty = 13/20 ticks (0.8us/1.25us) for a 1
const T0H: u16 = 0x8000 | 6; // Duty 7/20 ticks (0.4us/1.25us) for a 0
const RES: u16 = 0x8000;

const LED_COUNT: usize = 34;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = embassy_nrf::config::Config::default();
    let p = embassy_nrf::init(config);

    let mut pwm_config = Config::default();
    pwm_config.sequence_load = SequenceLoad::Common;
    pwm_config.prescaler = Prescaler::Div1; // 16MHz
    pwm_config.max_duty = 20; // 1.25us (1s / 16Mhz * 20)
    pwm_config.ch0_drive = OutputDrive::HighDrive;
    let Ok(mut seq_pwm) = SequencePwm::new_1ch(p.PWM0, p.P0_06, pwm_config) else {
        return;
    };

    let mut cnt = 0;
    loop {
        let mut words = heapless::Vec::<u16, 1024>::from_slice(&[RES; 100]).unwrap();
        for i in 0..LED_COUNT {
            let color = match (cnt + i + 2) % 3 {
                0 => RGB8 { g: 5, r: 0, b: 0 },
                1 => RGB8 { g: 0, r: 5, b: 0 },
                2 => RGB8 { g: 0, r: 0, b: 5 },
                _ => RGB8 { g: 5, r: 5, b: 5 },
            };
            for bit in color
                .g
                .view_bits::<Msb0>()
                .iter()
                .chain(color.r.view_bits())
                .chain(color.b.view_bits())
            {
                words.push(if *bit { T1H } else { T0H }).unwrap();
            }
        }

        let seq_config = embassy_nrf::pwm::SequenceConfig::default();
        let sequencer = SingleSequencer::new(&mut seq_pwm, words.as_slice(), seq_config);
        let _ = sequencer.start(SingleSequenceMode::Times(1));

        Timer::after_millis(500).await;
        cnt += 1;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
