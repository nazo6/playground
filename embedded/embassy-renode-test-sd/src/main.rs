#![no_std]
#![no_main]

use core::{hint::black_box, mem};

use core::fmt::Write;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use nrf_softdevice::{self as _, raw, Softdevice};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut str = heapless::String::<512>::new();
    write!(&mut str, "panic: {:?}", info).unwrap();
    let b = str.as_str();
    core::hint::black_box(b);
    loop {}
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = {
        let mut config = embassy_nrf::config::Config::default();
        config.gpiote_interrupt_priority = embassy_nrf::interrupt::Priority::P2;
        config.time_interrupt_priority = embassy_nrf::interrupt::Priority::P2;
        config.lfclk_source = embassy_nrf::config::LfclkSource::ExternalXtal;
        config.hfclk_source = embassy_nrf::config::HfclkSource::ExternalXtal;
        config
    };
    let _p = embassy_nrf::init(config);

    let config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 16,
            rc_temp_ctiv: 2,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 6,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256 }),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
        }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 3,
            central_role_count: 3,
            central_sec_count: 0,
            _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: b"HelloRust" as *const u8 as _,
            current_len: 9,
            max_len: 9,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
                raw::BLE_GATTS_VLOC_STACK as u8,
            ),
        }),
        ..Default::default()
    };

    let sd = Softdevice::enable(&config);

    let mut i = 0;
    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}

use cortex_m::peripheral::SCB;
use cortex_m_rt::{exception, ExceptionFrame};

#[exception]
unsafe fn DefaultHandler(irqn: i16) -> ! {
    let scb = unsafe { &*SCB::PTR };
    let cfsr = scb.cfsr.read();
    black_box(cfsr);
    loop {}
}
