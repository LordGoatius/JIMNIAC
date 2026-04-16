use std::ffi::OsString;

use clap::Parser;

/// The JX_01 Assembler
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Config {
    #[arg(short, long, default_value = None)]
    output: Option<OsString>
}
