use std::io::{self, Read, Write};

use zeroize::Zeroizing;

use crate::error::Error;

pub fn read_password(prompt: &str) -> Result<Zeroizing<String>, Error> {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(prompt.as_bytes())?;
    stdout.flush()?;

    let password = rpassword::read_password()?;
    Ok(Zeroizing::new(password))
}

pub fn read_password_stdin() -> Result<Zeroizing<String>, Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(Zeroizing::new(buf.trim().to_string()))
}
