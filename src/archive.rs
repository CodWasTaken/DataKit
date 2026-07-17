use std::fs;
use std::path::Path;

use crate::cli::ArchiveArgs;
use crate::error::Error;

pub fn run(args: ArchiveArgs) -> Result<(), Error> {
    match args.command.as_deref().unwrap_or("list") {
        "list" => cmd_list(&args),
        "extract" => cmd_extract(&args),
        "create" => cmd_create(&args),
        other => Err(Error::Message(format!("unknown archive command '{other}'"))),
    }
}

fn cmd_list(args: &ArchiveArgs) -> Result<(), Error> {
    let data = fs::read(&args.archive)?;
    if args.archive.ends_with(".zip") {
        list_zip(&data)?;
    } else {
        list_tar(&data)?;
    }
    Ok(())
}

fn cmd_extract(args: &ArchiveArgs) -> Result<(), Error> {
    let data = fs::read(&args.archive)?;
    let dest = args.output.as_deref().unwrap_or(".");
    if args.archive.ends_with(".zip") {
        extract_zip(&data, dest)?;
    } else {
        extract_tar(&data, dest)?;
    }
    Ok(())
}

fn cmd_create(args: &ArchiveArgs) -> Result<(), Error> {
    if args.archive.ends_with(".zip") {
        create_zip(&args.archive, &args.paths)?;
    } else {
        create_tar(&args.archive, &args.paths)?;
    }
    Ok(())
}

fn list_tar(data: &[u8]) -> Result<(), Error> {
    let mut archive = tar::Archive::new(std::io::Cursor::new(data));
    for entry in archive
        .entries()
        .map_err(|e| Error::Message(e.to_string()))?
    {
        let entry = entry.map_err(|e| Error::Message(e.to_string()))?;
        let path = entry.path().map_err(|e| Error::Message(e.to_string()))?;
        let path_s = path.display().to_string();
        let size = entry.size();
        let kind = if entry.header().entry_type().is_dir() {
            "dir"
        } else {
            "file"
        };
        println!("{kind:4} {size:>10} {path_s}");
    }
    Ok(())
}

fn list_zip(data: &[u8]) -> Result<(), Error> {
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(data))
        .map_err(|e| Error::Message(format!("zip error: {e}")))?;
    for i in 0..archive.len() {
        let file = archive
            .by_index(i)
            .map_err(|e| Error::Message(format!("zip read error: {e}")))?;
        let name = file.name().to_string();
        let size = file.size();
        let kind = if file.is_dir() { "dir" } else { "file" };
        println!("{kind:4} {size:>10} {name}");
    }
    Ok(())
}

fn extract_tar(data: &[u8], dest: &str) -> Result<(), Error> {
    let mut archive = tar::Archive::new(std::io::Cursor::new(data));
    for entry in archive
        .entries()
        .map_err(|e| Error::Message(e.to_string()))?
    {
        let mut entry = entry.map_err(|e| Error::Message(e.to_string()))?;
        let path = entry.path().map_err(|e| Error::Message(e.to_string()))?;
        let dest_path = Path::new(dest).join(&path);

        if path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(Error::Message(format!("path traversal detected: {path:?}")));
        }

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        entry
            .unpack(&dest_path)
            .map_err(|e| Error::Message(e.to_string()))?;
    }
    println!("extracted to: {dest}");
    Ok(())
}

fn extract_zip(data: &[u8], dest: &str) -> Result<(), Error> {
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(data))
        .map_err(|e| Error::Message(format!("zip error: {e}")))?;
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| Error::Message(format!("zip read error: {e}")))?;
        let name = file.name().to_string();
        let dest_path = Path::new(dest).join(&name);

        if name.contains("..") || name.starts_with('/') {
            return Err(Error::Message(format!("path traversal detected: {name}")));
        }

        if file.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut out = fs::File::create(&dest_path)?;
            std::io::copy(&mut file, &mut out)?;
        }
    }
    println!("extracted to: {dest}");
    Ok(())
}

fn create_tar(path: &str, sources: &[String]) -> Result<(), Error> {
    use flate2::write::GzEncoder;
    use flate2::Compression;

    let file = fs::File::create(path)?;
    let encoder = GzEncoder::new(file, Compression::best());
    let mut builder = tar::Builder::new(encoder);

    for source in sources {
        let p = Path::new(source);
        if p.is_dir() {
            builder
                .append_dir_all(p, p)
                .map_err(|e| Error::Message(e.to_string()))?;
        } else {
            builder
                .append_path(p)
                .map_err(|e| Error::Message(e.to_string()))?;
        }
    }

    let encoder = builder
        .into_inner()
        .map_err(|e| Error::Message(e.to_string()))?;
    encoder
        .finish()
        .map_err(|e| Error::Message(e.to_string()))?;
    println!("created: {path}");
    Ok(())
}

fn create_zip(path: &str, sources: &[String]) -> Result<(), Error> {
    let file = fs::File::create(path)?;
    let mut zipw = zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::<()>::default()
        .compression_method(zip::CompressionMethod::Deflated);

    for source in sources {
        let p = Path::new(source);
        if p.is_dir() {
            for entry in walkdir::WalkDir::new(p) {
                let entry = entry.map_err(|e| Error::Message(e.to_string()))?;
                let name = entry
                    .path()
                    .strip_prefix(p.parent().unwrap_or(Path::new(".")))
                    .unwrap_or(entry.path())
                    .display()
                    .to_string();
                if entry.file_type().is_dir() {
                    zipw.add_directory(&name, options)
                        .map_err(|e| Error::Message(format!("zip error: {e}")))?;
                } else {
                    zipw.start_file(&name, options)
                        .map_err(|e| Error::Message(format!("zip error: {e}")))?;
                    let mut f = fs::File::open(entry.path())?;
                    std::io::copy(&mut f, &mut zipw)?;
                }
            }
        } else {
            let name = p.file_name().unwrap().to_string_lossy().to_string();
            zipw.start_file(&name, options)
                .map_err(|e| Error::Message(format!("zip error: {e}")))?;
            let mut f = fs::File::open(p)?;
            std::io::copy(&mut f, &mut zipw)?;
        }
    }
    zipw.finish()
        .map_err(|e| Error::Message(format!("zip error: {e}")))?;
    println!("created: {path}");
    Ok(())
}
