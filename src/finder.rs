use crate::cli::Args;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let files = collect_files(&args.path, args.recursive)?;
    let grouped_by_name = group_by_filename(&files);

    let mut duplicates_found = false;

    for group in grouped_by_name.values() {
        if group.len() < 2 {
            continue;
        }

        let grouped_by_hash = group_by_hash(group)?;

        for (hash, paths) in grouped_by_hash {
            if paths.len() < 2 {
                continue;
            }

            duplicates_found = true;
            println!("Duplicate group:");
            println!("Hash: {}", hash);

            for path in paths {
                println!("- {}", path.display());
            }

            println!();
        }
    }

    if !duplicates_found {
        println!("No duplicates found.");
    }

    Ok(())
}

fn collect_files(path: &Path, recursive: bool) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();

    if recursive {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                files.push(entry_path.to_path_buf());
            }
        }
    } else {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                files.push(entry_path);
            }
        }
    }

    Ok(files)
}

fn group_by_filename(files: &[PathBuf]) -> HashMap<String, Vec<PathBuf>> {
    let mut groups = HashMap::new();

    for path in files {
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            groups
                .entry(file_name.to_string())
                .or_insert_with(Vec::new)
                .push(path.clone());
        }
    }

    groups
}

fn group_by_hash(files: &[PathBuf]) -> Result<HashMap<String, Vec<PathBuf>>, Box<dyn Error>> {
    let mut groups = HashMap::new();

    for path in files {
        let hash = calculate_file_hash(path)?;

        groups
            .entry(hash)
            .or_insert_with(Vec::new)
            .push(path.clone());
    }

    Ok(groups)
}

fn calculate_file_hash(path: &Path) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
