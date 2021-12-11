use std::collections::HashMap;

use data_encoding::{BASE64, HEXLOWER};

pub fn hex2base64(input: &str) -> String {
    let decoded: Vec<u8> = HEXLOWER.decode(input.as_bytes()).unwrap();
    BASE64.encode(&decoded)
}

pub fn crack_single_byte_xor(hexinput: &str, most_common_character: u8) {
    let mut bytes = HEXLOWER.decode(hexinput.as_bytes()).unwrap();

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

/// XOR a with b in-place.
pub fn fixed_xor_mut(a: &mut [u8], b: &[u8]) {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        a[i] ^= b[i];
    }
}
