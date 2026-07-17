use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use sha2::{Digest, Sha256, Sha512};
use sha3::Sha3_256;

use crate::cli::HashArgs;
use crate::error::Error;

pub fn run(args: HashArgs) -> Result<(), Error> {
    let algorithm = args.algorithm.as_deref().unwrap_or("sha256");

    if args.manifest {
        return run_manifest(&args.data, algorithm);
    }
    if args.check {
        return run_check(&args.data);
    }

    let input = fs::read(&args.data).unwrap_or_else(|_| args.data.as_bytes().to_vec());
    let hash = compute_hash(&input, algorithm)?;
    println!("{hash}");
    Ok(())
}

fn compute_hash(input: &[u8], algorithm: &str) -> Result<String, Error> {
    match algorithm {
        "md5" => {
            eprintln!("warning: MD5 is insecure");
            Ok(format!("{:x}", md5::Md5::digest(input)))
        }
        "sha256" => Ok(format!("{:x}", Sha256::digest(input))),
        "sha512" => Ok(format!("{:x}", Sha512::digest(input))),
        "sha3-256" => Ok(format!("{:x}", Sha3_256::digest(input))),
        "blake3" => {
            let mut hasher = blake3::Hasher::new();
            hasher.update(input);
            Ok(hasher.finalize().to_string())
        }
        other => Err(Error::Message(format!("unknown hash algorithm '{other}'"))),
    }
}

fn run_manifest(path: &str, algorithm: &str) -> Result<(), Error> {
    let p = Path::new(path);
    let entries = if p.is_dir() {
        collect_files(p)?
    } else {
        vec![p.to_path_buf()]
    };

    let mut manifest: BTreeMap<String, String> = BTreeMap::new();
    for entry in &entries {
        let data = fs::read(entry)?;
        let hash = compute_hash(&data, algorithm)?;
        let rel = entry
            .strip_prefix(p.parent().unwrap_or(Path::new(".")))
            .unwrap_or(entry)
            .display()
            .to_string();
        manifest.insert(rel, hash);
    }

    for (path, hash) in &manifest {
        println!("{hash}  {path}");
    }
    Ok(())
}

fn run_check(path: &str) -> Result<(), Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut failed = 0;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(2, "  ").collect();
        if parts.len() != 2 {
            eprintln!("warning: skipping malformed line: {line}");
            continue;
        }
        let expected = parts[0].trim();
        let file_path = parts[1].trim();

        match fs::read(file_path) {
            Ok(data) => {
                let algo = guess_algorithm(expected);
                match compute_hash(&data, algo) {
                    Ok(actual) if actual == expected => {}
                    Ok(actual) => {
                        eprintln!("{file_path}: FAILED ({actual} != {expected})");
                        failed += 1;
                    }
                    Err(e) => {
                        eprintln!("{file_path}: ERROR ({e})");
                        failed += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("{file_path}: FAILED open ({e})");
                failed += 1;
            }
        }
    }

    if failed == 0 {
        println!("all checksums verified");
        Ok(())
    } else {
        eprintln!("{failed} checksum(s) did NOT match");
        Err(Error::Message("checksum verification failed".to_string()))
    }
}

fn guess_algorithm(hash: &str) -> &str {
    let len = hash.len();
    if len == 64 {
        "sha256"
    } else if len == 128 {
        "sha512"
    } else if len == 32 {
        "md5"
    } else {
        "sha256"
    }
}

fn collect_files(dir: &Path) -> Result<Vec<std::path::PathBuf>, Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            files.push(entry.path());
        }
    }
    files.sort();
    Ok(files)
}
