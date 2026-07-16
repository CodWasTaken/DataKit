use similar::{ChangeTag, TextDiff};

use crate::cli::DiffArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: DiffArgs) -> Result<(), Error> {
    let fmt_a = format::detect_format(&args.file_a);
    let fmt_b = format::detect_format(&args.file_b);
    let val_a = crate::convert::read_input(&args.file_a, fmt_a)?;
    let val_b = crate::convert::read_input(&args.file_b, fmt_b)?;

    let text_a = serde_json::to_string_pretty(&val_a)?;
    let text_b = serde_json::to_string_pretty(&val_b)?;

    if text_a == text_b {
        println!("files are identical");
        return Ok(());
    }

    let diff = TextDiff::from_lines(&text_a, &text_b);
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{sign}{change}");
    }

    Ok(())
}
