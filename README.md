# Duplicate Finder
A command-line tool written in Rust that detects duplicate files by comparing file names and verifying file contents with SHA-256 hashes.
The tool first groups files by name and then hashes only potential matches, making duplicate detection more efficient than hashing every file upfront.

## Features
- Find duplicate files by file name and content hash
- SHA-256 based content verification
- Recursive directory scanning
- Efficient grouping before hashing
- Simple command-line interface
- Clear terminal output for duplicate groups

## Tech Stack
- Rust
- `clap` for command-line argument parsing
- `walkdir` for recursive file discovery
- `sha2` for SHA-256 hashing

## Usage
Run the tool from the project directory:
```
cargo run -- --path <folder> [options]
```
Example:
```
cargo run -- --path ./testfiles --recursive
```
Run the built executable after building the project:
```
target/release/duplicate-finder.exe --path ./testfiles --recursive
```