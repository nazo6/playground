use defmt::{info, warn};
use embassy_futures::join::join;
use embassy_futures::select::select;
use embassy_time::Timer;
use rand_core::{CryptoRng, RngCore};
use trouble_host::prelude::*;

/// Max number of connections
const CONNECTIONS_MAX: usize = 2;

/// Max number of L2CAP channels.
const L2CAP_CHANNELS_MAX: usize = 8; // Signal + att

// GATT Server definition
#[gatt_server]
struct Server {
    hid_service: HidService,
}

mod hid_uuid {
    use trouble_host::prelude::BluetoothUuid16;
    pub const HID_SERVICE: BluetoothUuid16 = BluetoothUuid16::new(0x1812);
    pub const HID_INFO: BluetoothUuid16 = BluetoothUuid16::new(0x2a4a);
    pub const REPORT_MAP: BluetoothUuid16 = BluetoothUuid16::new(0x2a4b);
    pub const HID_CONTROL_POINT: BluetoothUuid16 = BluetoothUuid16::new(0x2a4c);
    pub const HID_REPORT: BluetoothUuid16 = BluetoothUuid16::new(0x2a4d);
    pub const PROTOCOL_MODE: BluetoothUuid16 = BluetoothUuid16::new(0x2a4e);
    pub const HID_REPORT_REF: BluetoothUuid16 = BluetoothUuid16::new(0x2908);
}

const KEYBOARD_DESC: [u8; 69] = [
    5u8, 1u8, 9u8, 6u8, 161u8, 1u8, 5u8, 7u8, 25u8, 224u8, 41u8, 231u8, 21u8, 0u8, 37u8, 1u8,
    117u8, 1u8, 149u8, 8u8, 129u8, 2u8, 25u8, 0u8, 41u8, 255u8, 38u8, 255u8, 0u8, 117u8, 8u8,
    149u8, 1u8, 129u8, 3u8, 5u8, 8u8, 25u8, 1u8, 41u8, 5u8, 37u8, 1u8, 117u8, 1u8, 149u8, 5u8,
    145u8, 2u8, 149u8, 3u8, 145u8, 3u8, 5u8, 7u8, 25u8, 0u8, 41u8, 221u8, 38u8, 255u8, 0u8, 117u8,
    8u8, 149u8, 6u8, 129u8, 0u8, 192u8,
];

#[gatt_service(uuid = hid_uuid::HID_SERVICE)]
pub(super) struct HidService {
    #[characteristic(uuid = hid_uuid::HID_INFO, read, value = [0x11, 0x01, 0x00, 0x03])]
    pub hid_info: [u8; 4],

    #[characteristic(uuid = hid_uuid::REPORT_MAP, read, value = KEYBOARD_DESC)]
    pub report_map: [u8; 69],

    #[characteristic(uuid = hid_uuid::HID_CONTROL_POINT, write_without_response)]
    pub control_point: u8,

    #[characteristic(uuid = hid_uuid::PROTOCOL_MODE, read, write_without_response, value = 1)]
    pub protocol_mode: u8,

    #[characteristic(uuid = hid_uuid::HID_REPORT, read, notify)]
    #[descriptor(uuid = hid_uuid::HID_REPORT_REF, read, value = [0, 1])]
    pub input_keyboard: [u8; 8],
}

/// Run the BLE stack.
pub async fn run<C, RNG, const L2CAP_MTU: usize>(controller: C, random_generator: &mut RNG)
where
    C: Controller,
    RNG: RngCore + CryptoRng,
{
    // Using a fixed "random" address can be useful for testing. In real scenarios, one would
    // use e.g. the MAC 6 byte array as the address (how to get that varies by the platform).
    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    info!("Our address = {}", address);

    let mut resources: HostResources<CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU> =
        HostResources::new();
    let stack = trouble_host::new(controller, &mut resources)
        .set_random_address(address)
        .set_random_generator_seed(random_generator);
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();

    info!("Starting advertising and GATT service");
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "TrouBLE",
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .unwrap();

    let _ = join(ble_task(runner), async {
        loop {
            match advertise("Trouble Example", &mut peripheral, &server).await {
                Ok(conn) => {
                    // set up tasks when the connection is established to a central, so they don't run when no one is connected.
                    let a = gatt_events_task(&server, &conn);
                    let b = custom_task(&server, &conn, &stack);
                    // run until any task ends (usually because the connection has been closed),
                    // then return to advertising state.
                    select(a, b).await;
                }
                Err(e) => {
                    let e = defmt::Debug2Format(&e);
                    panic!("[adv] error: {:?}", e);
                }
            }
        }
    })
    .await;
}

/// This is a background task that is required to run forever alongside any other BLE tasks.
///
/// ## Alternative
///
/// If you didn't require this to be generic for your application, you could statically spawn this with i.e.
///
/// ```rust,ignore
///
/// #[embassy_executor::task]
/// async fn ble_task(mut runner: Runner<'static, SoftdeviceController<'static>>) {
///     runner.run().await;
/// }
///
/// spawner.must_spawn(ble_task(runner));
/// ```
async fn ble_task<C: Controller>(mut runner: Runner<'_, C>) {
    loop {
        if let Err(e) = runner.run().await {
            let e = defmt::Debug2Format(&e);
            panic!("[ble_task] error: {:?}", e);
        }
    }
}

/// Stream Events until the connection closes.
///
/// This function will handle the GATT events and process them.
/// This is how we interact with read and write requests.
async fn gatt_events_task(server: &Server<'_>, conn: &GattConnection<'_, '_>) -> Result<(), Error> {
    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => {
                info!("[gatt] disconnected: {:?}", reason);
                break;
            }
            GattConnectionEvent::Gatt { event } => match event {
                Ok(event) => {
                    let result = match &event {
                        GattEvent::Read(event) => {
                            if conn.raw().encrypted() {
                                None
                            } else {
                                Some(AttErrorCode::INSUFFICIENT_ENCRYPTION)
                            }
                        }
                        GattEvent::Write(event) => {
                            if conn.raw().encrypted() {
                                None
                            } else {
                                Some(AttErrorCode::INSUFFICIENT_ENCRYPTION)
                            }
                        }
                    };

                    // This step is also performed at drop(), but writing it explicitly is necessary
                    // in order to ensure reply is sent.
                    let result = if let Some(code) = result {
                        event.reject(code)
                    } else {
                        event.accept()
                    };
                    match result {
                        Ok(reply) => {
                            reply.send().await;
                        }
                        Err(e) => {
                            warn!("[gatt] error sending response: {:?}", e);
                        }
                    }
                }
                Err(e) => warn!("[gatt] error processing event: {:?}", e),
            },
            _ => {}
        }
    }
    info!("[gatt] task finished");
    Ok(())
}

/// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
async fn advertise<'a, 'b, C: Controller>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C>,
    server: &'b Server<'_>,
) -> Result<GattConnection<'a, 'b>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 31];
    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[[0x0f, 0x18]]),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..],
                scan_data: &[],
            },
        )
        .await?;
    info!("[adv] advertising");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    info!("[adv] connection established");
    Ok(conn)
}

async fn custom_task<C: Controller>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_>,
    stack: &Stack<'_, C>,
) {
    loop {
        // press `A`
        let _ = server
            .hid_service
            .input_keyboard
            .notify(conn, &[0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00])
            .await;
        // release `A`
        let _ = server
            .hid_service
            .input_keyboard
            .notify(conn, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
            .await;
        Timer::after_secs(2).await;
    }
}
