#![no_std]
#![no_main]

use defmt::info;
use defmt_embassy_usb_logger as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf_esb::{
    ptx::{new_ptx, PtxConfig},
    RadioConfig,
};
use esb_test::{init, Irqs, Message};
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = init(spawner, 0xCAFE, 0xBBBB);

    defmt::info!("ptx start");
    let (mut task, ptx) =
        new_ptx::<_, 255>(p.RADIO, Irqs, RadioConfig::default(), PtxConfig::default());

    join(task.run(), async move {
        loop {
            info!("Starting test");

            ptx.send(1, &Message::TestStart(100).encode(), false).await;
            embassy_time::Timer::after_millis(500).await;

            for i in 1..=100 {
                ptx.send(1, &Message::TestCount(i).encode(), false).await;
                // embassy_time::Timer::after_millis(5).await;
            }
            embassy_time::Timer::after_millis(500).await;

            ptx.send(1, &Message::TestEnd.encode(), false).await;
            info!("Test complete");

            embassy_time::Timer::after_millis(1000).await;
        }
    })
    .await;
}
