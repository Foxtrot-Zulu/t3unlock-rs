//! t3unlock-rs: Linux-first Rust CLI scaffold to unlock Samsung Portable SSD T3.
//!
//! This is a scaffold with a **dry-run** capable flow and a separable USB layer.
//! The actual vendor/protocol constants must be verified on a real device.
//!
//! See `docs/protocol.md` for notes and TODOs.

mod cli;
mod errors;
mod logging;
mod usb;

use anyhow::Result;
use cli::{Cli, Commands};
use tracing::{error, info};

fn main() {
    if let Err(e) = real_main() {
        error!(error = %e, "fatal error");
        std::process::exit(1);
    }
}

fn real_main() -> Result<()> {
    logging::init();
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Status { vid, pid, json } => {
            let dev = usb::DeviceSelector::from_cli(vid, pid);
            let status = usb::status(&dev)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&status)?);
            } else {
                println!("Device: {}", status.device_label);
                println!("Present: {}", status.present);
                println!("Locked:  {}", status.locked.unwrap_or(false));
            }
        }
        Commands::Unlock { vid, pid, password, dry_run, timeout_ms } => {
            let dev = usb::DeviceSelector::from_cli(vid, pid);
            let mut secret = password.unwrap_or_else(|| rpassword::prompt_password("Enter drive password: ").unwrap_or_default());
            let res = usb::unlock(&dev, secret.as_bytes(), dry_run, timeout_ms);
            zeroize::Zeroize::zeroize(&mut secret);
            res?;
            if dry_run {
                info!("DRY RUN: simulated unlock complete.");
            } else {
                println!("Unlock complete.");
            }
        }
        Commands::Doctor {} => {
            let report = usb::doctor()?;
            println!("{}", report);
        }
        Commands::GenCompletions { shell } => {
            cli::gen_completions(shell)?;
        }
        Commands::GenMan { out } => {
            cli::gen_man(out)?;
        }
    }
    Ok(())
}
