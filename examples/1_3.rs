//! Set 1 challenge 3: Single-byte XOR cipher
use std::collections::HashMap;

use data_encoding::HEXLOWER;

fn main() {
    let hexinput = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let mut bytes = HEXLOWER.decode(hexinput.as_bytes()).unwrap();

    // Assume that the space is the most common character in English text.
    // Find the byte that occurs the most.
    let most_common_character = b' ';

    // First, create a histogram
    let mut byte_counts = HashMap::new();
    for byte in &bytes {
        *byte_counts.entry(byte).or_insert(0) += 1;
    }

    // Find the most common byte
    let mut max_count = (0, 0); // (byte, count)
    for (byte, count) in byte_counts.into_iter() {
        if count > max_count.1 {
            println!(
                "Hist: Byte {} is more common than {} ({})",
                byte, max_count.0, count
            );
            max_count.0 = *byte;
            max_count.1 = count;
        }
    }

    println!("Most common byte: {:#02x}", max_count.0);
    let key = max_count.0 ^ most_common_character;
    println!(
        "The key must be: {:#02x} ^ {:#02x} = {:#02x}",
        max_count.0, most_common_character, key
    );

    print!("Input:  ");
    for byte in &bytes {
        print!("{:02x}", byte);
    }
    println!();

    print!("Output: ");
    for byte in &mut bytes {
        *byte ^= key;
        print!("{:02x}", byte);
    }
    println!();

    println!("Output string: {:?}", std::str::from_utf8(&bytes));
}
