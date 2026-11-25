//! Parser and canonical serializer for Zig Object Notation (ZON) state files.

use std::fmt::Write as _;

use thiserror::Error;

/// Dynamic ZON value preserving struct field order.
#[derive(Debug, Clone, PartialEq)]
pub enum ZonValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    EnumLiteral(String),
    Array(Vec<ZonValue>),
    Struct(Vec<(String, ZonValue)>),
}

impl ZonValue {
    /// Returns a struct field value by name if `self` is a `Struct`.
    pub fn get(&self, key: &str) -> Option<&ZonValue> {
        match self {
            Self::Struct(fields) => fields.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Returns the string slice if `self` is `Str`.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Returns the integer value if `self` is `Int`.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns the boolean value if `self` is `Bool`.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

/// Errors encountered while parsing ZON text.
#[derive(Debug, Error, PartialEq, Eq)]
#[error("ZON parse error at byte {offset}: {message}")]
pub struct ZonParseError {
    pub offset: usize,
    pub message: String,
}

/// Parses a ZON document into a [`ZonValue`].
pub fn parse(input: &str) -> Result<ZonValue, ZonParseError> {
    let mut parser = Parser {
        src: input,
        bytes: input.as_bytes(),
        pos: 0,
    };
    let val = parser.parse_value()?;
    parser.skip_ws_and_comments();
    if parser.pos != parser.bytes.len() {
        return Err(parser.err("unexpected trailing tokens"));
    }
    Ok(val)
}

/// Serializes a [`ZonValue`] into Zig-style indented ZON text.
pub fn stringify(value: &ZonValue) -> String {
    let mut out = String::new();
    write_value(value, 0, &mut out);
    out
}

fn write_value(value: &ZonValue, indent: usize, out: &mut String) {
    match value {
        ZonValue::Null => out.push_str("null"),
        ZonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        ZonValue::Int(n) => {
            let _ = write!(out, "{n}");
        }
        ZonValue::Float(f) => {
            let _ = write!(out, "{f}");
        }
        ZonValue::Str(s) => write_escaped_str(s, out),
        ZonValue::EnumLiteral(name) => {
            out.push('.');
            out.push_str(name);
        }
        ZonValue::Array(items) => {
            if items.is_empty() {
                out.push_str(".{}");
                return;
            }
            out.push_str(".{\n");
            for item in items {
                push_indent(indent + 4, out);
                write_value(item, indent + 4, out);
                out.push_str(",\n");
            }
            push_indent(indent, out);
            out.push('}');
        }
        ZonValue::Struct(fields) => {
            if fields.is_empty() {
                out.push_str(".{}");
                return;
            }
            out.push_str(".{\n");
            for (k, v) in fields {
                push_indent(indent + 4, out);
                out.push('.');
                if is_bare_ident(k) {
                    out.push_str(k);
                } else {
                    out.push_str("@\"");
                    for ch in k.chars() {
                        match ch {
                            '\\' => out.push_str("\\\\"),
                            '"' => out.push_str("\\\""),
                            _ => out.push(ch),
                        }
                    }
                    out.push('"');
                }
                out.push_str(" = ");
                write_value(v, indent + 4, out);
                out.push_str(",\n");
            }
            push_indent(indent, out);
            out.push('}');
        }
    }
}

fn push_indent(n: usize, out: &mut String) {
    for _ in 0..n {
        out.push(' ');
    }
}

fn is_bare_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn write_escaped_str(s: &str, out: &mut String) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(out, "\\x{:02x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

struct Parser<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn err(&self, message: impl Into<String>) -> ZonParseError {
        ZonParseError {
            offset: self.pos,
            message: message.into(),
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        self.bytes.get(self.pos + 1).copied()
    }

    fn skip_ws_and_comments(&mut self) {
        while let Some(b) = self.peek() {
            if b.is_ascii_whitespace() {
                self.pos += 1;
            } else if b == b'/' && self.peek_next() == Some(b'/') {
                self.pos += 2;
                while let Some(c) = self.peek() {
                    self.pos += 1;
                    if c == b'\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    fn parse_value(&mut self) -> Result<ZonValue, ZonParseError> {
        self.skip_ws_and_comments();
        let Some(b) = self.peek() else {
            return Err(self.err("unexpected end of input"));
        };

        match b {
            b'.' => {
                if self.peek_next() == Some(b'{') {
                    self.parse_container()
                } else {
                    self.pos += 1;
                    let ident = self.parse_identifier()?;
                    Ok(ZonValue::EnumLiteral(ident))
                }
            }
            b'"' => self.parse_string().map(ZonValue::Str),
            b'\\' if self.peek_next() == Some(b'\\') => {
                self.parse_multiline_string().map(ZonValue::Str)
            }
            b'-' | b'0'..=b'9' => self.parse_number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let word = self.parse_bare_word()?;
                match word {
                    "true" => Ok(ZonValue::Bool(true)),
                    "false" => Ok(ZonValue::Bool(false)),
                    "null" => Ok(ZonValue::Null),
                    other => Err(self.err(format!("unexpected identifier '{other}'"))),
                }
            }
            other => Err(self.err(format!("unexpected byte '{}'", other as char))),
        }
    }

    fn parse_container(&mut self) -> Result<ZonValue, ZonParseError> {
        self.pos += 2; // skip ".{"
        self.skip_ws_and_comments();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(ZonValue::Struct(Vec::new()));
        }

        // Distinguish struct (starts with `.field =`) vs array.
        let is_struct = self.looks_like_struct_entry();
        if is_struct {
            let mut fields = Vec::new();
            loop {
                self.skip_ws_and_comments();
                if self.peek() == Some(b'}') {
                    self.pos += 1;
                    break;
                }
                if self.peek() != Some(b'.') {
                    return Err(self.err("expected '.' before struct field name"));
                }
                self.pos += 1;
                let key = self.parse_identifier()?;
                self.skip_ws_and_comments();
                if self.peek() != Some(b'=') {
                    return Err(self.err("expected '=' after struct field name"));
                }
                self.pos += 1;
                let val = self.parse_value()?;
                fields.push((key, val));

                self.skip_ws_and_comments();
                match self.peek() {
                    Some(b',') => self.pos += 1,
                    Some(b'}') => {
                        self.pos += 1;
                        break;
                    }
                    _ => return Err(self.err("expected ',' or '}' in struct")),
                }
            }
            Ok(ZonValue::Struct(fields))
        } else {
            let mut items = Vec::new();
            loop {
                self.skip_ws_and_comments();
                if self.peek() == Some(b'}') {
                    self.pos += 1;
                    break;
                }
                items.push(self.parse_value()?);
                self.skip_ws_and_comments();
                match self.peek() {
                    Some(b',') => self.pos += 1,
                    Some(b'}') => {
                        self.pos += 1;
                        break;
                    }
                    _ => return Err(self.err("expected ',' or '}' in array")),
                }
            }
            Ok(ZonValue::Array(items))
        }
    }

    fn looks_like_struct_entry(&self) -> bool {
        let saved = self.pos;
        let mut probe = Parser {
            src: self.src,
            bytes: self.bytes,
            pos: saved,
        };
        if probe.peek() != Some(b'.') {
            return false;
        }
        probe.pos += 1;
        if probe.parse_identifier().is_err() {
            return false;
        }
        probe.skip_ws_and_comments();
        probe.peek() == Some(b'=')
    }

    fn parse_identifier(&mut self) -> Result<String, ZonParseError> {
        if self.peek() == Some(b'@') && self.peek_next() == Some(b'"') {
            self.pos += 1;
            return self.parse_string();
        }
        self.parse_bare_word().map(str::to_string)
    }

    fn parse_bare_word(&mut self) -> Result<&'a str, ZonParseError> {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.pos == start {
            return Err(self.err("expected identifier"));
        }
        Ok(&self.src[start..self.pos])
    }

    fn parse_string(&mut self) -> Result<String, ZonParseError> {
        self.pos += 1; // opening '"'
        let mut out = String::new();
        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if b == b'"' {
                self.pos += 1;
                return Ok(out);
            }
            if b == b'\\' {
                self.pos += 1;
                let Some(esc) = self.peek() else {
                    return Err(self.err("unterminated escape"));
                };
                self.pos += 1;
                match esc {
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    b'\\' => out.push('\\'),
                    b'"' => out.push('"'),
                    b'\'' => out.push('\''),
                    b'x' => {
                        let hex = self.take_hex(2)?;
                        let byte = u8::from_str_radix(hex, 16)
                            .map_err(|_| self.err("invalid \\x escape"))?;
                        out.push(byte as char);
                    }
                    b'u' => {
                        if self.peek() != Some(b'{') {
                            return Err(self.err("expected '{' in \\u{...}"));
                        }
                        self.pos += 1;
                        let start = self.pos;
                        while self.peek().is_some_and(|c| c != b'}') {
                            self.pos += 1;
                        }
                        if self.peek() != Some(b'}') {
                            return Err(self.err("unterminated \\u{...}"));
                        }
                        let hex = &self.src[start..self.pos];
                        self.pos += 1;
                        let cp = u32::from_str_radix(hex, 16)
                            .map_err(|_| self.err("invalid unicode escape"))?;
                        let ch =
                            char::from_u32(cp).ok_or_else(|| self.err("invalid unicode scalar"))?;
                        out.push(ch);
                    }
                    _ => return Err(self.err("unknown escape sequence")),
                }
            } else {
                let rest = &self.src[self.pos..];
                let ch = rest
                    .chars()
                    .next()
                    .ok_or_else(|| self.err("invalid UTF-8 in string"))?;
                out.push(ch);
                self.pos += ch.len_utf8();
            }
        }
        Err(self.err("unterminated string literal"))
    }

    fn parse_multiline_string(&mut self) -> Result<String, ZonParseError> {
        let mut out = String::new();
        let mut first = true;
        loop {
            if !(self.peek() == Some(b'\\') && self.peek_next() == Some(b'\\')) {
                break;
            }
            self.pos += 2;
            if !first {
                out.push('\n');
            }
            first = false;
            let start = self.pos;
            while let Some(b) = self.peek() {
                if b == b'\n' || b == b'\r' {
                    break;
                }
                self.pos += 1;
            }
            out.push_str(&self.src[start..self.pos]);
            if self.peek() == Some(b'\r') {
                self.pos += 1;
            }
            if self.peek() == Some(b'\n') {
                self.pos += 1;
            }
            self.skip_ws_and_comments();
        }
        Ok(out)
    }

    fn take_hex(&mut self, count: usize) -> Result<&'a str, ZonParseError> {
        if self.pos + count > self.bytes.len() {
            return Err(self.err("truncated hex escape"));
        }
        let s = &self.src[self.pos..self.pos + count];
        self.pos += count;
        Ok(s)
    }

    fn parse_number(&mut self) -> Result<ZonValue, ZonParseError> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'+' | b'-') {
                if (b == b'+' || b == b'-')
                    && !matches!(
                        self.bytes.get(self.pos - 1),
                        Some(b'e' | b'E' | b'p' | b'P')
                    )
                {
                    break;
                }
                self.pos += 1;
            } else {
                break;
            }
        }
        let token = self.src[start..self.pos].replace('_', "");
        if let Some(hex) = token.strip_prefix("0x") {
            let n = i64::from_str_radix(hex, 16)
                .map_err(|_| self.err(format!("invalid hex integer '{token}'")))?;
            return Ok(ZonValue::Int(n));
        }
        if let Some(hex) = token.strip_prefix("-0x") {
            let n = i64::from_str_radix(hex, 16)
                .map_err(|_| self.err(format!("invalid hex integer '{token}'")))?;
            return Ok(ZonValue::Int(-n));
        }
        if token.contains('.') || token.contains('e') || token.contains('E') {
            let f = token
                .parse::<f64>()
                .map_err(|_| self.err(format!("invalid float '{token}'")))?;
            return Ok(ZonValue::Float(f));
        }
        let n = token
            .parse::<i64>()
            .map_err(|_| self.err(format!("invalid integer '{token}'")))?;
        Ok(ZonValue::Int(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips_account_zon() {
        let sample = concat!(
            ".{\n",
            "    .user_id = \"597938510\",\n",
            "    .user_name = \"U607938510A\",\n",
            "    .token = \"6267c841-cf8f-3895-b051-3f9794f4ce6b\",\n",
            "    .sex = 1,\n",
            "}\n",
        );
        let parsed = parse(sample).expect("parse account zon");
        assert_eq!(
            parsed.get("user_id").and_then(ZonValue::as_str),
            Some("597938510")
        );
        assert_eq!(parsed.get("sex").and_then(ZonValue::as_i64), Some(1));
        let rendered = stringify(&parsed);
        assert_eq!(format!("{rendered}\n"), sample);
    }

    #[test]
    fn parses_nested_structs_and_arrays() {
        let doc = r#"
        .{
            .ok = true,
            .empty = .{},
            .items = .{ 1, -2, 3.5, .none },
        }
        "#;
        let v = parse(doc).expect("parse nested");
        assert_eq!(v.get("ok").and_then(ZonValue::as_bool), Some(true));
    }
}
