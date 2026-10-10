use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "dataset-detective",
    about = "Find and fix problems in CSV files"
)]
struct Cli {
    file: PathBuf,
}

fn load_csv(path: &Path) -> (Vec<String>, Vec<Vec<String>>) {
    let mut reader = csv::Reader::from_path(path).expect("could not open the file");

    let headers: Vec<String> = reader
        .headers()
        .expect("could not read the header row")
        .iter()
        .map(|h| h.to_string())
        .collect();

    let mut rows: Vec<Vec<String>> = Vec::new();
    for record in reader.records() {
        let record = record.expect("could not read a row");
        let row: Vec<String> = record.iter().map(|v| v.to_string()).collect();
        rows.push(row);
    }
    (headers, rows)
}

fn main() {
    let cli = Cli::parse();
    let (headers, rows) = load_csv(&cli.file);

    println!("Rows: {}", rows.len());
    println!("Columns: {}", headers.len());
    println!("Column names: {}", headers.join(", "));
}
