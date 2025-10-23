#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;
use log::info;

pub mod tasks;

//TODO: call the reset from esp_idf_sys in the panic handler
// #[panic_handler]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 98767); // Why do we
                                                                                       // need to
                                                                                       // declare
                                                                                       // this
                                                                                       // section?

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    //DHT_PIN
    let dht_pin = Output::new(
        peripherals.GPIO32,
        esp_hal::gpio::Level::High,
        OutputConfig::default(),
    );

    let _ = spawner;
    //TODO: spawn embassy_net runner, wifi task to reconnect in case of disconnection, mqtt client
    //listenig for input of the other tasks, dht task, anemo task (direction), wind speed and rain
    //content. 1 channel to receive strings and publish them.

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}
