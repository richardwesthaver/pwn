use agent::cfg::INSTANCE_ID;
use single_instance::SingleInstance;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  if cfg!(profile="release") {
    let instance = SingleInstance::new(INSTANCE_ID)?;
    if !instance.is_single() {
      return Ok(());
    }
  }
  let cfg = agent::init::init_and_install()?;
  let mut srv = agent::Service::new("127.0.0.1:8053".parse()?, None, cfg)?;
  srv.start().await?
}
