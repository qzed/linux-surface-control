use crate::cli::Command as DynCommand;
use crate::sys;

use anyhow::{Context, Result};


pub struct Command;

impl DynCommand for Command {
    fn name(&self) -> &'static str {
        "dtx"
    }

    fn build(&self) -> clap::App<'static, 'static> {
        use clap::{AppSettings, SubCommand};

        SubCommand::with_name(self.name())
            .about("Control the latch/dtx-system on the Surface Book 2")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("lock")
                .about("Lock the latch")
                .display_order(1))
            .subcommand(SubCommand::with_name("unlock")
                .about("Unlock the latch")
                .display_order(2))
            .subcommand(SubCommand::with_name("request")
                .about("Request latch-open or abort if already in progress")
                .display_order(3))
            .subcommand(SubCommand::with_name("get-base")
                .about("Get information about the currently attached base")
                .display_order(4))
            .subcommand(SubCommand::with_name("get-devicemode")
                .about("Query the current device operation mode")
                .display_order(4))
            .subcommand(SubCommand::with_name("get-latchstatus")
                .about("Query the current latch status")
                .display_order(4))
    }

    fn execute(&self, m: &clap::ArgMatches) -> Result<()> {
        match m.subcommand() {
            ("lock",            Some(m)) => self.lock(m),
            ("unlock",          Some(m)) => self.unlock(m),
            ("request",         Some(m)) => self.request(m),
            ("get-base",        Some(m)) => self.get_base_info(m),
            ("get-devicemode",  Some(m)) => self.get_device_mode(m),
            ("get-latchstatus", Some(m)) => self.get_latch_status(m),
            _                            => unreachable!(),
        }
    }
}

impl Command {
    fn lock(&self, m: &clap::ArgMatches) -> Result<()> {
        sys::dtx::Device::open()
            .context("Failed to open DTX device")?
            .latch_lock()
            .context("Failed to lock latch")?;

        if !m.is_present("quiet") {
            println!("Clipboard latch locked");
        }

        Ok(())
    }

    fn unlock(&self, m: &clap::ArgMatches) -> Result<()> {
        sys::dtx::Device::open()
            .context("Failed to open DTX device")?
            .latch_unlock()
            .context("Failed to unlock latch")?;

        if !m.is_present("quiet") {
            println!("Clipboard latch unlocked");
        }

        Ok(())
    }

    fn request(&self, m: &clap::ArgMatches) -> Result<()> {
        sys::dtx::Device::open()
            .context("Failed to open DTX device")?
            .latch_request()
            .context("Failed to send latch request")?;

        if !m.is_present("quiet") {
            println!("Clipboard latch request executed");
        }

        Ok(())
    }

    fn get_base_info(&self, m: &clap::ArgMatches) -> Result<()> {
        let info = sys::dtx::Device::open()
            .context("Failed to open DTX device")?
            .get_base_info()
            .context("Failed to get base info")?;

        if !m.is_present("quiet") {
            println!("State:       {}", info.state);
            println!("Device-Type: {}", info.device_type);
            println!("ID:          {:#04x}", info.id);
        } else {
            if let sys::dtx::DeviceType::Unknown(ty) = info.device_type {
                println!("{{ \"state\": \"{}\", \"type\": {}, \"id\": {} }}",
                         info.state, ty, info.id);
            } else {
                println!("{{ \"state\": \"{}\", \"type\": \"{}\", \"id\": {} }}",
                         info.state, info.device_type, info.id);
            }
        }

        Ok(())
    }

    fn get_device_mode(&self, m: &clap::ArgMatches) -> Result<()> {
        let mode = sys::dtx::Device::open()
            .context("Failed to open DTX device")?
            .get_device_mode()
            .context("Failed to get device mode")?;

        if !m.is_present("quiet") {
            println!("Device is in '{}' mode", mode);
        } else {
            println!("{}", mode);
        }

        Ok(())
    }

    fn get_latch_status(&self, m: &clap::ArgMatches) -> Result<()> {
        let status = sys::dtx::Device::open()
            .context("Failed to open DTX device")?
            .get_latch_status()
            .context("Failed to get latch status")?;

        if !m.is_present("quiet") {
            println!("Latch has been '{}'", status);
        } else {
            println!("{}", status);
        }

        Ok(())
    }
}
