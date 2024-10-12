use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Read the contents of the input file
    let contents = fs::read_to_string("level1_1.in")?;

    // Count the number of 'O' characters
    let count = contents.chars().filter(|&c| c == 'O').count();

    // Write the count to the output file
    let mut output_file = fs::File::create("level1_1.out")?;
    write!(output_file, "{}", count)?;

    println!("Count of 'O' characters: {}", count);
    Ok(())
}