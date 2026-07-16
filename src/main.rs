use clap::Parser;

mod cli;
mod completions;
mod convert;
mod count;
mod diff;
mod error;
mod filter;
mod format;
mod inspect;
mod query;
mod rename;
mod select;
mod sort;
mod stats;
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
        cli::Command::Stats(args) => stats::run(args),
        cli::Command::Filter(args) => filter::run(args),
        cli::Command::Select(args) => select::run(args),
        cli::Command::Diff(args) => diff::run(args),
        cli::Command::Sort(args) => sort::run(args),
        cli::Command::Count(args) => count::run(args),
        cli::Command::Rename(args) => rename::run(args),
    }
}
