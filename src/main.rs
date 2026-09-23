use anyhow::Result;
use clap::Parser;
use rand::Rng;
use std::iter;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Length of the random string
    #[arg(short, long, default_value_t = 32)]
    length: usize,

    /// Number of strings to generate
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Exclude lowercase characters
    #[arg(long)]
    no_lower: bool,

    /// Exclude uppercase characters
    #[arg(long)]
    no_upper: bool,

    /// Exclude numbers
    #[arg(long)]
    no_numbers: bool,

    /// Include symbols (disabled by default)
    #[arg(short, long)]
    symbols: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut charset = Vec::new();
    if !args.no_lower {
        charset.extend_from_slice(LOWERCASE);
    }
    if !args.no_upper {
        charset.extend_from_slice(UPPERCASE);
    }
    if !args.no_numbers {
        charset.extend_from_slice(NUMBERS);
    }
    if args.symbols {
        charset.extend_from_slice(SYMBOLS);
    }

    if charset.is_empty() {
        eprintln!("Error: Character set is empty! You excluded everything.");
        std::process::exit(1);
    }

    let mut rng = rand::thread_rng();

    for _ in 0..args.count {
        let s: String = iter::repeat(())
            .map(|()| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .take(args.length)
            .collect();
        println!("{}", s);
    }

    Ok(())
}
