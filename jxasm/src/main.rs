use clap::Parser;

use crate::config::Config;

mod config;
mod ast;
mod preprocessor;

fn main() {
    let config = Config::parse();
}
