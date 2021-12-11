//! Set 1 challenge 3: Single-byte XOR cipher

use cryptopals::crack_single_byte_xor;

fn main() {
    let hexinput = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    // Assume that the space is the most common character in English text.
    // Find the byte that occurs the most.
    let most_common_character = b' ';

    crack_single_byte_xor(hexinput, most_common_character);
}
