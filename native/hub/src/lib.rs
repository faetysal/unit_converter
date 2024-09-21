//! This `hub` crate is the
//! entry point of the Rust logic.

mod common;
mod messages;

use crate::common::*;
use tokio; // Comment this line to target the web.
// use tokio_with_wasm::alias as tokio; // Uncomment this line to target the web.

use unit_converter::*;

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
  tokio::spawn(handle_conversion());
}

async fn handle_conversion() -> Result<()> {
    use messages::convert::*;
    
    let mut receiver = Convert::get_dart_signal_receiver()?;
    while let Some(dart_signal) = receiver.recv().await {
      let message: Convert = dart_signal.message;
      let result: f64 = match message.quantity.as_str() {
        "length" => {
          convert::<LengthUnit>(
            message.value, 
            &message.from, 
            &message.to
          )
        },
        "time" => {
          convert::<TimeUnit>(
            message.value,
            &message.from,
            &message.to
          )
        },
        "area" => {
          convert::<AreaUnit>(
            message.value, 
            &message.from,
            &message.to
          )
        },
        "temperature" => {
          convert::<TemperatureUnit>(
            message.value,
            &message.from,
            &message.to
          )
        },
        "volume" => {
          convert::<VolumeUnit>(
            message.value,
            &message.from,
            &message.to
          )
        },
        "mass" => {
          convert::<MassUnit>(
            message.value,
            &message.from,
            &message.to
          )
        },
        "data" => {
          convert::<DataUnit>(
            message.value,
            &message.from,
            &message.to
          )
        },
        "speed" => {
          convert::<SpeedUnit>(
            message.value,
            &message.from,
            &message.to
          )
        },
        _ => panic!("Inavlid quantity type")
      };

      rinf::debug_print!("Rust Result: {result:?}");

      ConvertResult { value: result }.send_signal_to_dart();
    }
    Ok(())
}

fn convert<T: QuantityUnit>(value: f64, from_sym: &str, to_sym: &str) -> f64 {
  let from_unit = T::with_value(value, from_sym);
  let to_unit = T::from_symbol(to_sym);

  from_unit.to(to_unit).value()
}