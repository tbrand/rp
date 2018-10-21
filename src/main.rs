#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate failure;
extern crate rand;
extern crate rp_core;

mod config;
mod mode;
mod prelude;

use config::Config;
use mode::Mode;
use prelude::*;
use rp_core::cli;
use rp_core::copier::prelude::*;
use rp_core::error::Result;

fn main() -> Result<()> {
    env_logger::init();

    let matches = clap::App::new("rp")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(cli::f())
        .arg(cli::r())
        .arg(cli::mode())
        .arg(cli::files())
        .get_matches();

    let config = Config::new(matches);
    let src = config.src()?;
    let target = config.target()?;

    pattern(&src, target)?;

    let bytes = match config.mode()? {
        Mode::Seq => cp_all::<Seq>(&src, target, config.is_f(), config.is_r())?,
        Mode::Fut => cp_all::<Fut>(&src, target, config.is_f(), config.is_r())?,
        Mode::Fut2 => cp_all::<Fut2>(&src, target, config.is_f(), config.is_r())?,
    };

    info!("Copied {} bytes", bytes);

    Ok(())
}
