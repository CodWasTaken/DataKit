use clap::Parser;

mod archive;
mod atomic;
mod base32;
mod base58;
mod base64;
mod base85;
mod check;
mod cli;
mod completions;
mod compress;
mod convert;
mod count;
mod decompress;
mod decrypt;
mod dedup;
mod diff;
mod encrypt;
mod entries;
mod error;
mod explode;
mod fill;
mod filter;
mod flatten;
mod format;
mod hash;
mod head;
mod hex;
mod inspect;
mod keygen;
mod keys;
mod length;
mod merge;
mod pick;
mod pretty;
mod query;
mod rename;
mod reverse;
mod round;
mod sample;
mod search;
mod secret;
mod select;
mod shuffle;
mod sign;
mod slice;
mod sort;
mod stats;
mod tail;
mod unique;
mod validate;
mod values;
mod verify;
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
        cli::Command::Keys(args) => keys::run(args),
        cli::Command::Values(args) => values::run(args),
        cli::Command::Round(args) => round::run(args),
        cli::Command::Hash(args) => hash::run(args),
        cli::Command::Base64(args) => base64::run(args),
        cli::Command::Hex(args) => hex::run(args),
        cli::Command::Base32(args) => base32::run(args),
        cli::Command::Base58(args) => base58::run(args),
        cli::Command::Base85(args) => base85::run(args),
        cli::Command::Pretty(args) => pretty::run(args),
        cli::Command::Check(args) => check::run(args),
        cli::Command::Encrypt(args) => encrypt::run(args),
        cli::Command::Decrypt(args) => decrypt::run(args),
        cli::Command::Keygen(args) => keygen::run(args),
        cli::Command::Sign(args) => sign::run(args),
        cli::Command::Verify(args) => verify::run(args),
        cli::Command::Compress(args) => compress::run(args),
        cli::Command::Decompress(args) => decompress::run(args),
        cli::Command::Archive(args) => archive::run(args),
        cli::Command::Search(args) => search::run(args),
        cli::Command::Length(args) => length::run(args),
    }
}
