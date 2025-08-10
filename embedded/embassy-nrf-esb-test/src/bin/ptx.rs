#![no_std]
#![no_main]

use defmt::debug;
use defmt_embassy_usb_logger as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf_esb::{
    ptx::{new_ptx, PtxConfig},
    RadioConfig,
};
use esb_test::{init, Irqs};
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = init(spawner, 0xCAFE, 0xBBBB);

    defmt::info!("ptx start");
    let (mut task, ptx) =
        new_ptx::<_, 255>(p.RADIO, Irqs, RadioConfig::default(), PtxConfig::default());

    join(task.run(), async move {
        let mut i = 0;
        loop {
            debug!("Sending packet with value: {}", i);
            if let Err(e) = ptx.send(0, &[0, 1, 2, i], true).await {
                debug!("Send error: {:?}", e);
            }
            embassy_time::Timer::after_millis(1000).await;
            i += 1;
        }
    })
    .await;
}
