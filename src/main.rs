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

#[derive(Debug)]
enum ColumnType {
    Integer,
    Float,
    Text,
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

fn missing_percent(rows: &[Vec<String>], col: usize) -> f64 {
    if rows.is_empty() {
        return 0.0;
    }

    let mut missing: usize = 0;

    for row in rows {
        if row.get(col).map_or(true, |v| v.trim().is_empty()) {
            missing += 1;
        }
    }

    (missing as f64 / rows.len() as f64) * 100.0
}

fn infer_type(rows: &[Vec<String>], col: usize) -> ColumnType {
    let mut total = 0;
    let mut ints = 0;
    let mut numbers = 0;

    for row in rows {
        if let Some(v) = row.get(col) {
            let v = v.trim();
            if v.is_empty() {
                continue;
            }
            total += 1;
            if v.parse::<i64>().is_ok() {
                ints += 1;
                numbers += 1;
            } else if v.parse::<f64>().map_or(false, |x| x.is_finite()) {
                numbers += 1;
            }
        }
    }

    if total == 0 {
        return ColumnType::Text;
    }

    let ratio = numbers as f64 / total as f64;
    if ratio < 0.9 {
        ColumnType::Text
    } else if ints == numbers {
        ColumnType::Integer
    } else {
        ColumnType::Float
    }
}

fn main() {
    let cli = Cli::parse();
    let (headers, rows) = load_csv(&cli.file);

    for (i, name) in headers.iter().enumerate() {
        println!(
            "{}: {:?}, {:.1}% missing",
            name,
            infer_type(&rows, i),
            missing_percent(&rows, i)
        );
    }

    println!("Rows: {}", rows.len());
    println!("Columns: {}", headers.len());
    println!("Column names: {}", headers.join(", "));
}
