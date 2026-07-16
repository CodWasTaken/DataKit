use serde::Serialize;

use crate::cli::PrettyArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: PrettyArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    let value = crate::convert::read_input(&args.data, input_fmt)?;
    let indent = args.indent.unwrap_or(2);

    if indent == 0 {
        serde_json::to_writer(std::io::stdout(), &value)?;
    } else {
        let indent_bytes = vec![b' '; indent];
        let fmt = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
        let mut ser = serde_json::Serializer::with_formatter(std::io::stdout(), fmt);
        value.serialize(&mut ser)?;
    }
    println!();
    Ok(())
}
