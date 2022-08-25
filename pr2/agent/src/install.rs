use crate::cfg;
use std::{env, fs, path::PathBuf};

fn copy_executable() -> Result<PathBuf, crate::Error> {
  let current_exe = env::current_exe()?;

  let install_dir = cfg::get_agent_directory()?;
  fs::create_dir_all(&install_dir)?;

  let install_target = cfg::get_agent_install_target()?;
  fs::copy(current_exe, &install_target)?;

  Ok(install_target)
}

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::install;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::install;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::install;
