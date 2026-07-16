use clap::Parser;

mod cli;
mod convert;
mod error;
mod format;
mod inspect;
mod query;
mod validate;

fn main() -> Result<(), error::Error> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Inspect(args) => inspect::run(args),
        cli::Command::Convert(args) => convert::run(args),
        cli::Command::Validate(args) => validate::run(args),
        cli::Command::Query(args) => query::run(args),
    }
}
