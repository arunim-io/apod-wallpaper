use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cli {}

pub fn get_cli() -> Cli {
    return Cli::parse();
}
