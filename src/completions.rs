use clap::CommandFactory;
use clap_complete::{generate, Shell};

use crate::cli;
use crate::error::Error;

pub fn run(shell: Shell) -> Result<(), Error> {
    let mut cmd = cli::Cli::command();
    generate(shell, &mut cmd, "datakit", &mut std::io::stdout());
    Ok(())
}
