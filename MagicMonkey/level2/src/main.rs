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
    let currencies: usize = lines.next().unwrap()?.parse().unwrap();
    _ = lines.next().unwrap();
    _ = lines.next().unwrap();

    for _ in 0..currencies {
        let coin_values: Vec<u32> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let amounts: Vec<u32> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for amount in amounts {
            let (coin1, coin2) = find_coin_combination(&coin_values, amount);
            writeln!(output, "{} {}", coin1, coin2)?;
        }
    }

    Ok(())
}

fn find_coin_combination(coins: &[u32], amount: u32) -> (u32, u32) {
    let valid_coins: Vec<u32> = coins.into_iter().filter(|c| **c < amount).cloned().collect();
    
    for coin1 in valid_coins.iter() {
        for coin2 in valid_coins.iter() {
            if coin1 + coin2 == amount {
                return (*coin1, *coin2);
            }
        }
    }
    (0, 0)  // Return (0, 0) if no combination is found
}