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
    /// Flatten nested objects
    Flatten(FlattenArgs),
    /// Extract a slice of array records
    Slice(SliceArgs),
    /// Random sample of records
    Sample(SampleArgs),
    /// Shuffle array records randomly
    Shuffle(ShuffleArgs),
    /// First N records of an array
    Head(HeadArgs),
    /// Last N records of an array
    Tail(TailArgs),
    /// Reverse an array
    Reverse(ReverseArgs),
    /// Fill null values in a field
    Fill(FillArgs),
    /// Explode array field into multiple records
    Explode(ExplodeArgs),
    /// Remove duplicate records
    Dedup(DedupArgs),
    /// Convert object to key-value entries
    Entries(EntriesArgs),
    /// Merge two data files
    Merge(MergeArgs),
    /// Pick a random record
    Pick(PickArgs),
    /// Zip two arrays together
    Zip(ZipArgs),
    /// List keys of an object
    Keys(KeysArgs),
    /// List values of an object
    Values(ValuesArgs),
    /// Round numbers to a given precision
    Round(RoundArgs),
    /// Hash data content
    Hash(HashArgs),
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

#[derive(Args)]
pub struct FlattenArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Separator between nested keys (default ".")
    #[arg(short, long)]
    pub sep: Option<String>,
}

#[derive(Args)]
pub struct SliceArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Start index
    #[arg(short, long)]
    pub start: Option<usize>,
    /// End index (exclusive)
    #[arg(short, long)]
    pub end: Option<usize>,
}

#[derive(Args)]
pub struct SampleArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Number of records to sample
    #[arg(short, long)]
    pub count: Option<usize>,
    /// Random seed for reproducibility
    #[arg(short, long)]
    pub seed: Option<u64>,
}

#[derive(Args)]
pub struct ShuffleArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Random seed for reproducibility
    #[arg(short, long)]
    pub seed: Option<u64>,
}

#[derive(Args)]
pub struct HeadArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Number of records to show (default 10)
    #[arg(short, long)]
    pub count: Option<usize>,
}

#[derive(Args)]
pub struct TailArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Number of records to show (default 10)
    #[arg(short, long)]
    pub count: Option<usize>,
}

#[derive(Args)]
pub struct ReverseArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
}

#[derive(Args)]
pub struct FillArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Field name to fill nulls in
    #[arg(short, long)]
    pub field: String,
    /// Value to fill with
    #[arg(short, long)]
    pub value: String,
}

#[derive(Args)]
pub struct ExplodeArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Array field to explode
    #[arg(short, long)]
    pub field: String,
}

#[derive(Args)]
pub struct DedupArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Field to check for duplicates (all fields if omitted)
    #[arg(short, long)]
    pub field: Option<String>,
}

#[derive(Args)]
pub struct EntriesArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
}

#[derive(Args)]
pub struct MergeArgs {
    /// First data file
    pub file_a: String,
    /// Second data file
    pub file_b: String,
}

#[derive(Args)]
pub struct PickArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Random seed for reproducibility
    #[arg(short, long)]
    pub seed: Option<u64>,
}

#[derive(Args)]
pub struct ZipArgs {
    /// First array file
    pub file_a: String,
    /// Second array file
    pub file_b: String,
}

#[derive(Args)]
pub struct KeysArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
}

#[derive(Args)]
pub struct ValuesArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
}

#[derive(Args)]
pub struct RoundArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Number of decimal places
    #[arg(short, long)]
    pub decimals: Option<u32>,
}

#[derive(Args)]
pub struct HashArgs {
    /// Path to the data file (use "-" for stdin)
    pub data: String,
    /// Hash algorithm (md5)
    #[arg(short, long)]
    pub algorithm: Option<String>,
}
