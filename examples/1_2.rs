/// Set 1 challenge 2: Fixed XOR
use data_encoding::HEXLOWER;

use cryptopals::fixed_xor_mut;

fn main() {
    let mut buf1 = HEXLOWER
        .decode(b"1c0111001f010100061a024b53535009181c")
        .unwrap();
    let buf2 = HEXLOWER
        .decode(b"686974207468652062756c6c277320657965")
        .unwrap();
    fixed_xor_mut(&mut buf1, &buf2);
    assert_eq!(
        HEXLOWER.encode(&buf1),
        "746865206b696420646f6e277420706c6179"
    );
    println!("OK");
}
