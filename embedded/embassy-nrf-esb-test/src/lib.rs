#![no_std]

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    config::{Config, HfclkSource},
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
    let mut config = Config::default();
    // NOTE: Don't remove this line, it is required to make wireless work.
    config.hfclk_source = HfclkSource::ExternalXtal;
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

#[derive(defmt::Format, Debug)]
pub enum Message {
    TestStart(u8),
    TestCount(u8),
    TestEnd,
}

impl Message {
    pub fn encode(&self) -> [u8; 4] {
        match self {
            Message::TestStart(count) => [0x01, *count, 0x00, 0x00],
            Message::TestCount(count) => [0x02, *count, 0x00, 0x00],
            Message::TestEnd => [0x03, 0x00, 0x00, 0x00],
        }
    }
    pub fn decode(buf: &[u8]) -> Option<Self> {
        if buf.len() < 3 {
            return None; // Not enough data to decode
        }
        match buf[0] {
            0x01 => Some(Message::TestStart(buf[1])),
            0x02 => Some(Message::TestCount(buf[1])),
            0x03 => Some(Message::TestEnd),
            _ => None,
        }
    }
}
