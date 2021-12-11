//! Set 1 challenge 4: Detect single-character XOR
use cryptopals::crack_single_byte_xor;
use data_encoding::HEXLOWER;
use histogram::Histogram;

fn histogram(hexstr: &str) -> Histogram {
    let mut histogram = Histogram::new();
    for byte in HEXLOWER.decode(hexstr.as_bytes()).unwrap() {
        histogram.increment(byte as u64).unwrap();
    }
    histogram
}

fn main() -> Result<(), std::io::Error> {
    // Load file
    let input = include_str!("1_4.txt");

    // Parse lines
    let mut lowest_variance = (String::new(), 999999);
    for line in input.lines() {
        let trimmed = line.trim();
        let h = histogram(trimmed);
        let variance = h.stdvar().unwrap();
        if variance < lowest_variance.1 {
            println!("Variance of {} is lower ({})", trimmed, variance);
            lowest_variance.0 = trimmed.to_string();
            lowest_variance.1 = variance;
        }
    }

    // Crack
    println!("Cracking...");
    crack_single_byte_xor(&lowest_variance.0, b' ');

    Ok(())
}
