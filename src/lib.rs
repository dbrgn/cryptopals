use data_encoding::{BASE64, HEXLOWER};

pub fn hex2base64(input: &str) -> String {
    let decoded: Vec<u8> = HEXLOWER.decode(input.as_bytes()).unwrap();
    BASE64.encode(&decoded)
}
