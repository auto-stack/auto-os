//! Strict framing + finite-JSON codec for `json-f01/1` (Design02 §4).
//!
//! Frame = `u32 LE payload_len` + UTF-8 JSON payload. The length header is
//! validated before any payload allocation; zero length is `invalid_frame`,
//! over-limit is `resource_exhausted`. The payload parser is a hand-written
//! recursive-descent JSON reader that enforces the finite profile:
//!
//! | violation                          | classification        |
//! |------------------------------------|-----------------------|
//! | non-UTF-8 bytes / JSON syntax / trailing content / empty object shape | `invalid_frame` |
//! | duplicate key                      | `schema_mismatch`     |
//! | number not a canonical u32 (negative/float/exponent/overflow) | `schema_mismatch` |
//! | unknown `op` / unknown key / missing key / wrong type | `schema_mismatch` |
//! | container depth > 16               | `resource_exhausted`  |
//!
//! Encoding is serde-based (the DTO surface is the single shape source);
//! decoding goes through `JVal` and per-op allowed-key tables, so admission
//! semantics never depend on serde's tolerance.

use crate::error::CommError;
use crate::protocol::Message;

/// Canonical u32 on the wire; every other number form is rejected.
#[derive(Debug, Clone, PartialEq)]
pub enum JVal {
    Obj(Vec<(String, JVal)>),
    Arr(Vec<JVal>),
    Str(String),
    Bool(bool),
    Uint(u32),
}

#[derive(Debug, Clone, PartialEq)]
enum PErr {
    BadUtf8,
    Syntax,
    Depth,
    DuplicateKey,
    NumberForm,
}

impl PErr {
    fn classify(self) -> CommError {
        match self {
            PErr::BadUtf8 => CommError::invalid_frame("payload is not valid UTF-8"),
            PErr::Syntax => CommError::invalid_frame("malformed JSON payload"),
            PErr::Depth => CommError::resource("JSON container depth exceeds profile limit"),
            PErr::DuplicateKey => CommError::schema("duplicate object key"),
            PErr::NumberForm => CommError::schema("number is not a canonical u32"),
        }
    }
}

struct Parser<'a> {
    s: &'a str,
    b: &'a [u8],
    i: usize,
}

const MAX_DEPTH: usize = 16;

impl<'a> Parser<'a> {
    fn new(bytes: &'a [u8]) -> Result<Self, PErr> {
        let s = std::str::from_utf8(bytes).map_err(|_| PErr::BadUtf8)?;
        Ok(Parser { s, b: bytes, i: 0 })
    }

    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.i += 1;
        }
    }

    fn eat(&mut self, lit: &str) -> Result<(), PErr> {
        if self.b[self.i..].starts_with(lit.as_bytes()) {
            self.i += lit.len();
            Ok(())
        } else {
            Err(PErr::Syntax)
        }
    }

    fn parse_value(&mut self, depth: usize) -> Result<JVal, PErr> {
        if depth > MAX_DEPTH {
            return Err(PErr::Depth);
        }
        self.skip_ws();
        match self.peek() {
            Some(b'{') => self.parse_object(depth),
            Some(b'[') => self.parse_array(depth),
            Some(b'"') => Ok(JVal::Str(self.parse_string()?)),
            Some(b't') => {
                self.eat("true")?;
                Ok(JVal::Bool(true))
            }
            Some(b'f') => {
                self.eat("false")?;
                Ok(JVal::Bool(false))
            }
            Some(b'n') => Err(PErr::NumberForm), // `null` is not in the profile
            Some(c) if c == b'-' || c.is_ascii_digit() => self.parse_u32(),
            _ => Err(PErr::Syntax),
        }
    }

    fn parse_object(&mut self, depth: usize) -> Result<JVal, PErr> {
        self.i += 1; // '{'
        let mut entries: Vec<(String, JVal)> = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.i += 1;
            return Ok(JVal::Obj(entries));
        }
        loop {
            self.skip_ws();
            if self.peek() != Some(b'"') {
                return Err(PErr::Syntax);
            }
            let key = self.parse_string()?;
            self.skip_ws();
            if self.peek() != Some(b':') {
                return Err(PErr::Syntax);
            }
            self.i += 1;
            let val = self.parse_value(depth + 1)?;
            if entries.iter().any(|(k, _)| *k == key) {
                return Err(PErr::DuplicateKey);
            }
            entries.push((key, val));
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.i += 1,
                Some(b'}') => {
                    self.i += 1;
                    return Ok(JVal::Obj(entries));
                }
                _ => return Err(PErr::Syntax),
            }
        }
    }

    fn parse_array(&mut self, depth: usize) -> Result<JVal, PErr> {
        self.i += 1; // '['
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.i += 1;
            return Ok(JVal::Arr(items));
        }
        loop {
            let v = self.parse_value(depth + 1)?;
            items.push(v);
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.i += 1,
                Some(b']') => {
                    self.i += 1;
                    return Ok(JVal::Arr(items));
                }
                _ => return Err(PErr::Syntax),
            }
        }
    }

    fn parse_string(&mut self) -> Result<String, PErr> {
        self.i += 1; // opening quote
        let mut out = String::new();
        loop {
            match self.peek() {
                None => return Err(PErr::Syntax),
                Some(b'"') => {
                    self.i += 1;
                    return Ok(out);
                }
                Some(b'\\') => {
                    self.i += 1;
                    let esc = self.peek().ok_or(PErr::Syntax)?;
                    self.i += 1;
                    match esc {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let hi = self.parse_hex4()?;
                            let ch = if (0xD800..0xDC00).contains(&hi) {
                                // surrogate pair
                                if self.peek() != Some(b'\\') {
                                    return Err(PErr::Syntax);
                                }
                                self.i += 1;
                                if self.peek() != Some(b'u') {
                                    return Err(PErr::Syntax);
                                }
                                self.i += 1;
                                let lo = self.parse_hex4()?;
                                if !(0xDC00..0xE000).contains(&lo) {
                                    return Err(PErr::Syntax);
                                }
                                let c = 0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00);
                                char::from_u32(c).ok_or(PErr::Syntax)?
                            } else if (0xDC00..0xE000).contains(&hi) {
                                return Err(PErr::Syntax); // lone low surrogate
                            } else {
                                char::from_u32(hi).ok_or(PErr::Syntax)?
                            };
                            out.push(ch);
                        }
                        _ => return Err(PErr::Syntax),
                    }
                }
                Some(c) if c < 0x20 => return Err(PErr::Syntax), // raw control char
                Some(_) => {
                    // consume one UTF-8 scalar (input is already valid UTF-8)
                    let rest = &self.s[self.i..];
                    let ch = rest.chars().next().ok_or(PErr::Syntax)?;
                    out.push(ch);
                    self.i += ch.len_utf8();
                }
            }
        }
    }

    fn parse_hex4(&mut self) -> Result<u32, PErr> {
        let mut v = 0u32;
        for _ in 0..4 {
            let c = self.peek().ok_or(PErr::Syntax)?;
            let d = (c as char).to_digit(16).ok_or(PErr::Syntax)?;
            v = v * 16 + d;
            self.i += 1;
        }
        Ok(v)
    }

    fn parse_u32(&mut self) -> Result<JVal, PErr> {
        let start = self.i;
        if self.peek() == Some(b'-') {
            return Err(PErr::NumberForm);
        }
        if self.peek() == Some(b'0') {
            self.i += 1;
            if matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                return Err(PErr::Syntax); // leading zero
            }
        } else {
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.i += 1;
            }
        }
        let digits = &self.s[start..self.i];
        // any fraction/exponent marker means "not a canonical u32"
        match self.peek() {
            Some(b'.' | b'e' | b'E' | b'+' | b'-') => return Err(PErr::NumberForm),
            _ => {}
        }
        digits
            .parse::<u32>()
            .map(JVal::Uint)
            .map_err(|_| PErr::NumberForm) // overflow
    }
}

/// Per-op allowed top-level keys (frozen wire shapes; see protocol.rs).
fn allowed_keys(op: &str) -> Option<&'static [&'static str]> {
    Some(match op {
        "hello" => &["op", "protocol_version", "codec", "token"],
        "welcome" => &["op", "broker_epoch", "principal", "role"],
        "register" => &[
            "op",
            "corr_id",
            "service_id",
            "interface",
            "schema_id",
            "descriptor",
        ],
        "registered" => &["op", "corr_id", "service_ref"],
        "discover" => &["op", "corr_id", "service_id"],
        "discovered" => &["op", "corr_id", "service_ref"],
        "call" => &[
            "op",
            "request_id",
            "trace_id",
            "service_ref",
            "method",
            "schema_id",
            "timeout_ms",
            "body",
        ],
        "pcall" => &[
            "op",
            "route_id",
            "request_id",
            "caller",
            "method",
            "schema_id",
            "deadline_ms",
            "body",
        ],
        "reply" => &["op", "request_id", "body"],
        "preply" => &["op", "route_id", "body"],
        "error" => &[
            "op",
            "corr_id",
            "request_id",
            "route_id",
            "code",
            "message",
            "execution",
            "domain_code",
        ],
        _ => return None,
    })
}

const SERVICE_REF_KEYS: &[&str] = &["broker_epoch", "instance_id", "service_id", "interface"];

fn check_keys(val: &JVal) -> Result<(), CommError> {
    let obj = match val {
        JVal::Obj(o) => o,
        _ => return Err(CommError::schema("message must be a JSON object")),
    };
    let op = match obj.iter().find(|(k, _)| k == "op") {
        Some((_, JVal::Str(s))) => s.clone(),
        _ => return Err(CommError::schema("missing string field `op`")),
    };
    let allowed =
        allowed_keys(&op).ok_or_else(|| CommError::schema(format!("unknown op `{op}`")))?;
    for (k, _) in obj {
        if !allowed.contains(&k.as_str()) {
            return Err(CommError::schema(format!(
                "unknown field `{k}` for op `{op}`"
            )));
        }
    }
    // nested object field checks for the two composite shapes
    if allowed.contains(&"service_ref") {
        if let Some((_, v)) = obj.iter().find(|(k, _)| k == "service_ref") {
            if let JVal::Obj(fields) = v {
                for (k, _) in fields {
                    if !SERVICE_REF_KEYS.contains(&k.as_str()) {
                        return Err(CommError::schema(format!(
                            "unknown field `{k}` in service_ref"
                        )));
                    }
                }
            } else {
                return Err(CommError::schema("service_ref must be an object"));
            }
        }
    }
    Ok(())
}

fn jval_to_value(v: JVal) -> serde_json::Value {
    match v {
        JVal::Obj(o) => serde_json::Value::Object(
            o.into_iter()
                .map(|(k, v)| (k, jval_to_value(v)))
                .collect::<serde_json::Map<String, serde_json::Value>>(),
        ),
        JVal::Arr(a) => serde_json::Value::Array(a.into_iter().map(jval_to_value).collect()),
        JVal::Str(s) => serde_json::Value::String(s),
        JVal::Bool(b) => serde_json::Value::Bool(b),
        JVal::Uint(u) => serde_json::Value::Number(u.into()),
    }
}

/// Build one wire frame: `u32 LE len` + payload.
pub fn frame_bytes(payload: &[u8]) -> Result<Vec<u8>, CommError> {
    if payload.len() as u64 > crate::admission::Limits::MAX_FRAME as u64 {
        return Err(CommError::resource("payload exceeds frame limit"));
    }
    let mut out = Vec::with_capacity(4 + payload.len());
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
    Ok(out)
}

/// Validate a length header before any payload allocation.
pub fn check_frame_header(len: u32) -> Result<usize, CommError> {
    if len == 0 {
        Err(CommError::invalid_frame("zero-length frame"))
    } else if len as u64 > crate::admission::Limits::MAX_FRAME as u64 {
        Err(CommError::resource("frame length exceeds profile limit"))
    } else {
        Ok(len as usize)
    }
}

/// Strict decode of one frame payload into a [`Message`].
pub fn decode(payload: &[u8]) -> Result<Message, CommError> {
    if payload.is_empty() {
        return Err(CommError::invalid_frame("zero-length frame"));
    }
    let mut p = Parser::new(payload).map_err(|e| e.classify())?;
    let val = p.parse_value(1).map_err(|e| e.classify())?;
    p.skip_ws();
    if p.i != p.b.len() {
        return Err(CommError::invalid_frame(
            "trailing content after JSON value",
        ));
    }
    check_keys(&val)?;
    let json = jval_to_value(val);
    serde_json::from_value::<Message>(json)
        .map_err(|e| CommError::schema(format!("message does not match profile: {e}")))
}

/// Encode a message into a frame (serde is the single shape source here).
pub fn encode(msg: &Message) -> Result<Vec<u8>, CommError> {
    let payload = serde_json::to_vec(msg)
        .map_err(|e| CommError::invalid_frame(format!("encode failed: {e}")))?;
    frame_bytes(&payload)
}
