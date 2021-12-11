use data_encoding::{BASE64, HEXLOWER};

pub fn hex2base64(input: &str) -> String {
    let decoded: Vec<u8> = HEXLOWER.decode(input.as_bytes()).unwrap();
    BASE64.encode(&decoded)
}

/// XOR a with b in-place.
pub fn fixed_xor_mut(a: &mut [u8], b: &[u8]) {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        a[i] ^= b[i];
    }
}
