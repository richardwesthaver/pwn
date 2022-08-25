mod cfg;
mod error;
mod init;
mod install;
pub use error::Error;

use cfg::INSTANCE_ID;
use single_instance::SingleInstance;
use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let instance = SingleInstance::new(INSTANCE_ID)?;
  if !instance.is_single() {
    return Ok(());
  }

  let _ = init::init_and_install()?;

  let one_sec = time::Duration::from_secs(1);
  loop {
    thread::sleep(one_sec);
  }
}
