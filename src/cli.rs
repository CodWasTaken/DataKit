use clap::{Args, Parser, Subcommand, ValueEnum};

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
    /// Generate shell completions
    Completions(CompletionsArgs),
    /// Compute summary statistics for numeric fields
    Stats(StatsArgs),
    /// Filter records by a condition
    Filter(FilterArgs),
    /// Select specific fields from data
    Select(SelectArgs),
    /// Show differences between two data files
    Diff(DiffArgs),
    /// Sort records by a field
    Sort(SortArgs),
    /// Count records in a data file
    Count(CountArgs),
    /// Rename a field in records
    Rename(RenameArgs),
    /// List unique values of a field
    Unique(UniqueArgs),
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ShellVariant {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}

impl From<ShellVariant> for clap_complete::Shell {
    fn from(v: ShellVariant) -> Self {
        match v {
            ShellVariant::Bash => clap_complete::Shell::Bash,
            ShellVariant::Elvish => clap_complete::Shell::Elvish,
            ShellVariant::Fish => clap_complete::Shell::Fish,
            ShellVariant::PowerShell => clap_complete::Shell::PowerShell,
            ShellVariant::Zsh => clap_complete::Shell::Zsh,
        }
    }
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
    /// Input format (auto-detected from extension if omitted)
    #[arg(long)]
    pub from: Option<String>,
    /// Output format (auto-detected from extension if omitted)
    #[arg(long)]
    pub to: Option<String>,
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

#[derive(Args)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    pub shell: ShellVariant,
}

#[derive(Args)]
pub struct StatsArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
}

#[derive(Args)]
pub struct FilterArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Filter condition: 'field op value' (e.g. 'age > 30')
    #[arg(short, long)]
    pub condition: String,
}

#[derive(Args)]
pub struct SelectArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Comma-separated field names to include
    #[arg(short, long)]
    pub fields: String,
}

#[derive(Args)]
pub struct DiffArgs {
    /// First file to compare
    pub file_a: String,
    /// Second file to compare
    pub file_b: String,
}

#[derive(Args)]
pub struct SortArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Field to sort by
    #[arg(short, long)]
    pub by: String,
    /// Sort in descending order
    #[arg(short, long)]
    pub desc: bool,
}

#[derive(Args)]
pub struct CountArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
}

#[derive(Args)]
pub struct RenameArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Field rename mapping: 'old:new'
    #[arg(short, long)]
    pub mapping: String,
}

#[derive(Args)]
pub struct UniqueArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Field to find unique values for
    #[arg(short, long)]
    pub field: String,
}
