use crate::cli::CheckArgs;
use crate::error::Error;
use crate::format;

pub fn run(args: CheckArgs) -> Result<(), Error> {
    let input_fmt = format::detect_format(&args.data);
    match crate::convert::read_input(&args.data, input_fmt) {
        Ok(_) => {
            println!("valid");
            Ok(())
        }
        Err(e) => {
            eprintln!("invalid: {e}");
            std::process::exit(1);
        }
    }
}
