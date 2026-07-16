use clap::Parser;

mod cli;
mod completions;
mod convert;
mod error;
mod format;
mod inspect;
mod query;
mod validate;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), error::Error> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Inspect(args) => inspect::run(args),
        cli::Command::Convert(args) => convert::run(args),
        cli::Command::Validate(args) => validate::run(args),
        cli::Command::Query(args) => query::run(args),
        cli::Command::Completions(args) => completions::run(args.shell.into()),
    }
}
