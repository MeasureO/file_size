use std::env;

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new(size_in_bytes: u64) -> Self {
        let bytes = format!("{} bytes", size_in_bytes);
        let kilobytes = format!("{:.2} kilobytes", size_in_bytes as f64 / 1000.0);
        let megabytes = format!("{:.2} megabytes", size_in_bytes as f64 / 1_000_000.0);
        let gigabytes = format!("{:.2} gigabytes", size_in_bytes as f64 / 1_000_000_000.0);

        Sizes {
            bytes,
            kilobytes,
            megabytes,
            gigabytes,
        }
    }

    fn from_input(size: u64, unit: &str) -> Self {
        let size_in_bytes = match unit.to_lowercase().as_str() {
            "bytes" | "b" => size,
            "kilobytes" | "kb" => size * 1000,
            "megabytes" | "mb" => size * 1_000_000,
            "gigabytes" | "gb" => size * 1_000_000_000,
            _ => panic!("Invalid unit"),
        };

        Sizes::new(size_in_bytes)
    }
}

fn parse_input(input: &str) -> Result<(u64, &str), &str> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Invalid input format. Expected format: <size> <unit>");
    }

    let size: u64 = parts[0].parse().map_err(|_| "Invalid number")?;
    let unit = parts[1];

    Ok((size, unit))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <size> <unit>", args[0]);
        return;
    }

    let input = &args[1];
    match parse_input(input) {
        Ok((size, unit)) => {
            let sizes = Sizes::from_input(size, unit);
            println!("{:?}", sizes);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
