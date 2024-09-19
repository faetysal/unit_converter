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
          let from_unit = LengthUnit::with_value(message.value, &message.from);
          let to_unit = LengthUnit::from_symbol(&message.to);

          from_unit.to(to_unit).value()
        },
        "time" => {
          let from_unit = TimeUnit::with_value(message.value, &message.from);
          let to_unit = TimeUnit::from_symbol(&message.to);

          from_unit.to(to_unit).value()
        },
        "area" => {
          let from_unit = AreaUnit::with_value(message.value, &message.from);
          let to_unit = AreaUnit::from_symbol(&message.to);

          from_unit.to(to_unit).value()
        },
        "temperature" => {
          let from_unit = TemperatureUnit::with_value(message.value, &message.from);
          let to_unit = TemperatureUnit::from_symbol(&message.to);

          from_unit.to(to_unit).value()
        },
        _ => panic!("Inavlid quantity type")
      };

      rinf::debug_print!("Rust Result: {result:?}");

      ConvertResult { value: result }.send_signal_to_dart();
    }
    Ok(())
}
