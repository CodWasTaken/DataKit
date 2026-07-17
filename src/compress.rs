use std::fs;

use crate::atomic;
use crate::cli::CompressArgs;
use crate::error::Error;

pub fn run(args: CompressArgs) -> Result<(), Error> {
    let data = fs::read(&args.data)?;
    let algorithm = args.algorithm.as_deref().unwrap_or("gzip");
    let level = args.level.unwrap_or(6);

    let compressed = match algorithm {
        "gzip" => {
            use flate2::write::GzEncoder;
            use flate2::Compression;
            let mut encoder = GzEncoder::new(Vec::new(), Compression::new(level as u32));
            std::io::Write::write_all(&mut encoder, &data)?;
            encoder.finish().map_err(Error::Io)?
        }
        "zstd" => zstd::encode_all(std::io::Cursor::new(&data), level)
            .map_err(|e| Error::Message(format!("zstd compression error: {e}")))?,
        other => {
            return Err(Error::Message(format!(
                "unknown compression algorithm '{other}'"
            )))
        }
    };

    let output_path = args
        .output
        .unwrap_or_else(|| format!("{}.{algorithm}", args.data));
    atomic::write(&output_path, &compressed)?;
    println!("compressed: {output_path}");
    Ok(())
}
