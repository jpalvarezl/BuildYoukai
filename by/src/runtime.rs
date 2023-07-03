use clap::Parser;
use std::{error::Error, path::PathBuf};

pub fn parse_from_cli() -> Runtime {
    Runtime::parse()
}

#[derive(Debug, Parser)]
#[command(
    name = "by",
    version = "0.1.0",
    author = "Jose Alvarez <jp.alvarezl@gmail.com>",
    about = "Build Youkai CLI",
    long_about = "None"
)]
pub struct Runtime {
    #[arg(
        short='i',
        long,
        value_hint = clap::ValueHint::DirPath)]
    pub input: PathBuf,
}
