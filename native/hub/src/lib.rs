//! This `hub` crate is the
//! entry point of the Rust logic.

mod common;
mod messages;

use crate::common::*;
use tokio; // Comment this line to target the web.
// use tokio_with_wasm::alias as tokio; // Uncomment this line to target the web.

use unit_converter::{self as UC, QuantityUnit};

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
      let minutes: UC::Time = UC::Time::from_mins(2_f64);
      let seconds: UC::Time = UC::Time::to_secs(minutes);

      rinf::debug_print!("{seconds:?}");

      ConvertResult { value: seconds.to_unit().value() }.send_signal_to_dart();
    }
    Ok(())
}
