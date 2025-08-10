#![no_std]
#![no_main]

use defmt::{debug, error, info};
use defmt_embassy_usb_logger as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf_esb::RadioConfig;
use esb_test::{init, Irqs};
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = init(spawner, 0xCAFE, 0xCCCC);

    defmt::info!("prx start");
    let (mut task, mut prx) =
        embassy_nrf_esb::prx::new_prx::<_, 255>(p.RADIO, Irqs, RadioConfig::default());
    join(task.run(), async move {
        let mut latest_recv: Option<u8> = None;
        loop {
            let mut buffer = [0u8; 255];
            match prx.recv(&mut buffer).await {
                Ok(n) => {
                    info!(
                        "Received packet with unexpected length: {}, contents: {:?}",
                        n,
                        &buffer[..n]
                    );
                    if buffer[0..3] != [0, 1, 2] {
                        error!(
                            "Received packet with unexpected header: {:?}",
                            &buffer[0..3]
                        );
                    } else {
                        let value = buffer[2];
                        if let Some(latest_recv) = latest_recv {
                            if value != latest_recv + 1 {
                                error!(
                                    "Received out-of-order packet: expected {}, got {}",
                                    latest_recv + 1,
                                    value
                                );
                            }
                        } else {
                            debug!("First packet received");
                        }
                        latest_recv = Some(value);
                    }
                }
                Err(e) => {
                    error!("Receive error: {:?}", e);
                }
            }
        }
    })
    .await;
}
