use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "datakit", about = "Toolkit for structured data")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Inspect the structure and schema of a data file
    Inspect(InspectArgs),
    /// Convert data between formats
    Convert(ConvertArgs),
}

#[derive(Args)]
pub struct InspectArgs {
    /// Path to the input file (reads from stdin if omitted)
    pub path: Option<String>,
}

#[derive(Args)]
pub struct ConvertArgs {
    /// Path to the input file (use "-" for stdin)
    pub input: String,
    /// Path to the output file (writes to stdout if omitted)
    pub output: Option<String>,
}
