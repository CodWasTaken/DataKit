use std::fs;

use crate::atomic;
use crate::cli::DecompressArgs;
use crate::error::Error;

pub fn run(args: DecompressArgs) -> Result<(), Error> {
    let data = fs::read(&args.data)?;
    let algorithm = args.algorithm.as_deref().unwrap_or("gzip");

    let decompressed = match algorithm {
        "gzip" => {
            use flate2::read::GzDecoder;
            use std::io::Read;
            let mut decoder = GzDecoder::new(std::io::Cursor::new(&data));
            let mut out = Vec::new();
            decoder.read_to_end(&mut out)?;
            out
        }
        "zstd" => zstd::decode_all(std::io::Cursor::new(&data))
            .map_err(|e| Error::Message(format!("zstd decompression error: {e}")))?,
        other => {
            return Err(Error::Message(format!(
                "unknown compression algorithm '{other}'"
            )))
        }
    };

    let output_path = args.output.unwrap_or_else(|| {
        args.data
            .strip_suffix(".gz")
            .or_else(|| args.data.strip_suffix(".zst"))
            .unwrap_or(&args.data)
            .to_string()
    });
    atomic::write(&output_path, &decompressed)?;
    println!("decompressed: {output_path}");
    Ok(())
}
