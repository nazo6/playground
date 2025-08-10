#![no_std]
#![no_main]

use defmt::{error, info};
use defmt_embassy_usb_logger as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf_esb::RadioConfig;
use esb_test::{init, Irqs, Message};
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = init(spawner, 0xCAFE, 0xCCCC);

    defmt::info!("prx start");
    let (mut task, mut prx) =
        embassy_nrf_esb::prx::new_prx::<_, 255>(p.RADIO, Irqs, RadioConfig::default());
    join(task.run(), async move {
        let mut test_data: Option<(u8, u8, u8)> = None;
        loop {
            let mut buffer = [0u8; 255];
            match prx.recv(&mut buffer).await {
                Ok(n) => {
                    let Some(message) = Message::decode(&buffer[..n]) else {
                        error!("Received invalid message");
                        continue;
                    };
                    match message {
                        Message::TestStart(max) => {
                            test_data = Some((max, 0, 0));
                        }
                        Message::TestCount(c) => {
                            if let Some((_, count, loss)) = &mut test_data {
                                if c != *count + 1 {
                                    error!("Missed data: {} to {}", count, c);
                                    *loss += 1;
                                }
                                *count = c;
                            } else {
                                error!("Received count without a test start");
                            }
                        }
                        Message::TestEnd => {
                            info!("Test result: {:?}", test_data);
                        }
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
