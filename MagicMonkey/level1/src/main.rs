use std::fs::{File, read_dir};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Read all *.in files in the ./data directory
    let data_dir = Path::new("./data");
    for entry in read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("in") {
            process_file(&path)?;
        }
    }
    Ok(())
}

fn process_file(input_path: &Path) -> io::Result<()> {
    // Create output file path
    let output_path = input_path.with_extension("out");
    
    let input = io::BufReader::new(File::open(input_path)?);
    let mut output = File::create(output_path)?;
    let mut lines = input.lines();

    // Read the number of data rows
    let n: usize = lines.next().unwrap()?.parse().unwrap();
    _ = lines.next();

    // Process each data row
    for line in lines.take(n) {
        let coins: Vec<u32> = line?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let result = find_first_unpayable_amount(&coins);
        writeln!(output, "{}", result)?;
    }

    Ok(())
}

fn find_first_unpayable_amount(coins: &[u32]) -> u32 {
    (1..).find(|&amount| !coins.contains(&amount)).unwrap()
}