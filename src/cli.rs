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
