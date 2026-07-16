use clap::Parser;

mod cli;
mod completions;
mod convert;
mod count;
mod dedup;
mod diff;
mod entries;
mod error;
mod explode;
mod fill;
mod filter;
mod flatten;
mod format;
mod head;
mod inspect;
mod merge;
mod pick;
mod query;
mod rename;
mod reverse;
mod sample;
mod select;
mod shuffle;
mod slice;
mod sort;
mod stats;
mod tail;
mod unique;
mod validate;
mod zip;

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
        cli::Command::Unique(args) => unique::run(args),
        cli::Command::Flatten(args) => flatten::run(args),
        cli::Command::Slice(args) => slice::run(args),
        cli::Command::Sample(args) => sample::run(args),
        cli::Command::Shuffle(args) => shuffle::run(args),
        cli::Command::Head(args) => head::run(args),
        cli::Command::Tail(args) => tail::run(args),
        cli::Command::Reverse(args) => reverse::run(args),
        cli::Command::Fill(args) => fill::run(args),
        cli::Command::Explode(args) => explode::run(args),
        cli::Command::Dedup(args) => dedup::run(args),
        cli::Command::Entries(args) => entries::run(args),
        cli::Command::Merge(args) => merge::run(args),
        cli::Command::Pick(args) => pick::run(args),
        cli::Command::Zip(args) => zip::run(args),
    }
}
