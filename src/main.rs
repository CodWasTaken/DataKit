use clap::Parser;

mod cli;
mod convert;
mod error;
mod inspect;

fn main() -> Result<(), error::Error> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Inspect(args) => inspect::run(args),
        cli::Command::Convert(args) => convert::run(args),
    }
}
