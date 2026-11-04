//! Random identifiers and token material.
//!
//! All protocol-visible IDs are the fixed 128-bit text form: 32 lowercase hex
//! chars. Tokens are the fixed 256-bit text form: 64 lowercase hex chars.
//! Tokens are credentials: they must never be logged or echoed.

use getrandom::getrandom;

pub const ID_LEN: usize = 32;
pub const TOKEN_LEN: usize = 64;

pub fn new_id() -> String {
    let mut b = [0u8; 16];
    getrandom(&mut b).expect("OS randomness unavailable");
    to_hex(&b)
}

pub fn new_token() -> String {
    let mut b = [0u8; 32];
    getrandom(&mut b).expect("OS randomness unavailable");
    to_hex(&b)
}

pub fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((b & 0xf) as u32, 16).unwrap());
    }
    s
}

/// True when `s` is exactly 32 lowercase hex chars (a 128-bit ID text form).
pub fn is_id_text(s: &str) -> bool {
    s.len() == ID_LEN && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// True when `s` is exactly 64 lowercase hex chars (a 256-bit token text form).
pub fn is_token_text(s: &str) -> bool {
    s.len() == TOKEN_LEN && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}
