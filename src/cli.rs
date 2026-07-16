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
    /// Validate data against a JSON Schema
    Validate(ValidateArgs),
    /// Query a field path from data
    Query(QueryArgs),
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
    /// Indent width for JSON output (0 = minified, default 2)
    #[arg(short, long, default_value = "2")]
    pub indent: usize,
}

#[derive(Args)]
pub struct ValidateArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Path to the JSON Schema file
    #[arg(short, long)]
    pub schema: String,
}

#[derive(Args)]
pub struct QueryArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Dot-separated field path (e.g. "user.name", "items[0].id")
    #[arg(short, long)]
    pub path: String,
}
