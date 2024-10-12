use std::fs::{read_dir, File};
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
        let mut coin_values: Vec<u32> = lines
            .next()
            .unwrap()?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        coin_values.sort();

        let amounts: Vec<u32> = lines
            .next()
            .unwrap()?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let lcm = least_common_multiple(&coin_values);
        let coin_combinations = build_coin_combinations(&coin_values, 2 * lcm as usize);

        for mut amount in amounts {
            let last_coin = coin_values.last().unwrap();
            let number_of_max_coins = if amount > 2 * lcm { (amount - amount % lcm - lcm) / last_coin } else { 0 };
            amount -= number_of_max_coins * last_coin;

            let mut coin_used = construct_amount_with_fewest_coins(&coin_combinations, amount);

            // Combine coins with the maximum number of coins. Check if the coin is the same as the last coin in the coin_used vector. If it is, add the amount to the last coin in the coin_used vector. If it is not, add the coin to the coin_used vector.
            let mut found = false;
            for coin in coin_used.iter_mut() {
                if coin.coin == *last_coin {
                    coin.amount += number_of_max_coins;
                    found = true;
                }
            }

            if !found {
                coin_used.insert(0, CoinWithAmount {
                    coin: *last_coin,
                    amount: number_of_max_coins,
                });
            }

            let output_string = coin_used
                .iter()
                .map(|coin| format!("{}x{}", coin.amount, coin.coin))
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(output, "{}", output_string)?;
        }
    }

    Ok(())
}

struct CoinWithAmount {
    coin: u32,
    amount: u32,
}

fn build_coin_combinations(coins: &Vec<u32>, max_amount: usize) -> Vec<usize> {
    let mut dp = vec![None; max_amount + 1];
    let mut coin_used = vec![0; max_amount + 1];

    dp[0] = Some(0);
    for i in 1..=max_amount {
        for &coin in coins {
            let coin = coin as usize;
            if coin <= i {
                if let Some(prev_count) = dp[i - coin] {
                    let new_count = prev_count + 1;
                    if dp[i].is_none() || new_count < dp[i].unwrap() {
                        dp[i] = Some(new_count);
                        coin_used[i] = coin;
                    }
                }
            }
        }
    }

    coin_used
}

fn construct_amount_with_fewest_coins(coin_used: &Vec<usize>, amount: u32) -> Vec<CoinWithAmount> {
    let amount = amount as usize;

    let mut result: Vec<CoinWithAmount> = Vec::new();
    let mut remaining = amount;
    while remaining > 0 {
        let coin = coin_used[remaining] as u32;
        if let Some(last) = result.last_mut() {
            if last.coin == coin {
                last.amount += 1;
            } else {
                result.push(CoinWithAmount { coin, amount: 1 });
            }
        } else {
            result.push(CoinWithAmount { coin, amount: 1 });
        }
        remaining -= coin as usize;
    }

    result.reverse();
    result
}

fn least_common_multiple(v: &[u32]) -> u32 {
    let mut lcm = v[0];
    for &num in v.iter().skip(1) {
        lcm = lcm * num / gcd(lcm, num);
    }
    lcm
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

