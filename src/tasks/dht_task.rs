use super::mqtt_task::{DEFAULT_STRING_SIZE, MQTT_CHANNEL};
use alloc::string::String;
use dht_sensor::dht22::r#async as dht22_async;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Delay;
use esp_hal::gpio::Output;
// use log::info;

#[embassy_executor::task]
pub async fn dht_task(
    dht_pin: Output<'static>,
    mqtt_channel: Channel<CriticalSectionRawMutex, String<DEFAULT_STRING_SIZE>, 5>,
) {
    let delay = Delay;
    info!("Starting the dht");
    dht_pin.set_high(); //TODO: Check if the pin doesnt need to be configured as input-output

    // loop {
    //     match dht22_async::read(&mut delay, &mut dht_pin).await {
    //         Ok(reading) => {
    //             info!(
    //                 "Got {}C {}%",
    //                 reading.temperature, reading.relative_humidity
    //             );
    //         }
    //         Err(e) => {
    //             error!("Fail reading DHT {e}")
    //         }
    //     }
    // }
}
