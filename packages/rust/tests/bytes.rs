//! Every byte value in each place of the grammar, and the boundaries of a string (CK-14.3).

use polyspec_http_cookies::{parse_cookie, set_cookie, Attributes, Error, SameSite, MAX_NAME_AND_VALUE, MAX_PATH};

fn plain(path: &str) -> Attributes {
    Attributes { path: path.to_owned(), max_age: None, secure: false, http_only: false, same_site: None }
}

fn read(line: &[u8]) -> Vec<(String, String)> {
    parse_cookie(&[line.to_vec()])
}

fn pair(name: &str, value: &str) -> Vec<(String, String)> {
    vec![(name.to_owned(), value.to_owned())]
}

/// The character whose code point is the byte: the byte of a Latin-1 text.
fn character(byte: u8) -> char {
    char::from(byte)
}

/// CK-7.1: the tchar of RFC 9110, section 5.6.2, listed apart from the code under test.
fn is_tchar(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte)
}

/// CK-8.2: the cookie-octet of RFC 6265, section 4.1.1, listed apart from the code under test.
fn is_cookie_octet(byte: u8) -> bool {
    (0x21..=0x7E).contains(&byte) && !b"\",;\\".contains(&byte)
}

#[test]
fn every_byte_of_a_cookie_value() {
    for byte in 0..=u8::MAX {
        let mut line = b"a=".to_vec();
        line.push(byte);
        line.extend_from_slice(b"x");
        let expected = match byte {
            // CK-3.1: the value ends at `;`.
            b';' => pair("a", ""),
            // CK-4.2: an ASCII byte is itself, `%` without two digits and `+` included.
            0x00..=0x7F => pair("a", &format!("{}x", character(byte))),
            // CK-4.7: a lone byte above 0x7F is not UTF-8.
            _ => vec![],
        };
        assert_eq!(read(&line), expected, "a=<{byte:#04x}>x");
    }
}

#[test]
fn every_byte_at_the_end_of_a_cookie_value() {
    for byte in 0..=u8::MAX {
        let mut line = b"a=x".to_vec();
        line.push(byte);
        let expected = match byte {
            // CK-3.2: spaces and tabs at the end of a pair are removed.
            b' ' | b'\t' | b';' => pair("a", "x"),
            0x00..=0x7F => pair("a", &format!("x{}", character(byte))),
            _ => vec![],
        };
        assert_eq!(read(&line), expected, "a=x<{byte:#04x}>");
    }
}

#[test]
fn every_byte_of_a_cookie_name() {
    for byte in 0..=u8::MAX {
        let line = [byte, b'=', b'1'];
        let expected = match byte {
            // CK-3.2 then CK-4.5: the name is empty after the space or tab is removed. CK-3.1 then CK-4.4 and CK-4.5:
            // `;` gives an empty pair and `=1`. CK-3.3 then CK-4.5: `==1` has an empty name.
            b' ' | b'\t' | b';' | b'=' => vec![],
            // CK-4.1: the name is its bytes.
            0x00..=0x7F => pair(&character(byte).to_string(), "1"),
            // CK-4.6: a lone byte above 0x7F is not UTF-8.
            _ => vec![],
        };
        assert_eq!(read(&line), expected, "<{byte:#04x}>=1");
    }
}

#[test]
fn every_percent_escape() {
    for byte in 0..=u8::MAX {
        for escape in [format!("%{byte:02X}"), format!("%{byte:02x}")] {
            let line = format!("a={escape}").into_bytes();
            // CK-4.2 and CK-4.7: the escape is its byte, and a lone byte above 0x7F is not UTF-8.
            let expected = if byte.is_ascii() { pair("a", &character(byte).to_string()) } else { vec![] };
            assert_eq!(read(&line), expected, "a={escape}");
        }
    }
}

#[test]
fn a_percent_at_each_place_of_the_end_of_a_value() {
    // CK-4.2: a `%` that two hexadecimal digits do not follow, also at the end of a line and before `;`.
    assert_eq!(read(b"a=%"), pair("a", "%"));
    assert_eq!(read(b"a=%4"), pair("a", "%4"));
    assert_eq!(read(b"a=%41"), pair("a", "A"));
    assert_eq!(read(b"a=%;b=1"), vec![("a".to_owned(), "%".to_owned()), ("b".to_owned(), "1".to_owned())]);
    assert_eq!(read(b"a=%4;b=1"), vec![("a".to_owned(), "%4".to_owned()), ("b".to_owned(), "1".to_owned())]);
    // CK-4.2: an escape of two digits is decoded once, and `%%41` is `%` then `A`.
    assert_eq!(read(b"a=%%41"), pair("a", "%A"));
}

#[test]
fn a_pair_at_the_start_and_at_the_end_of_a_line() {
    // CK-3.1: no `;` is needed before the first pair or after the last one.
    assert_eq!(read(b"a=1"), pair("a", "1"));
    assert_eq!(read(b";a=1"), pair("a", "1"));
    assert_eq!(read(b"a=1;"), pair("a", "1"));
    assert_eq!(read(b";"), vec![]);
    assert_eq!(read(b""), vec![]);
}

#[test]
fn every_byte_of_a_set_cookie_name() {
    for byte in 0..=u8::MAX {
        let name = character(byte).to_string();
        let result = set_cookie(&name, "v", &plain("/"));
        // CK-7.1 and CK-7.2: a name of one byte is a token when the byte is a tchar; a character above U+007F is
        // two UTF-8 bytes, neither of them a tchar.
        if is_tchar(byte) {
            assert_eq!(result, Ok(format!("{name}=v; Path=/").into_bytes()), "name {byte:#04x}");
        } else {
            assert_eq!(result, Err(Error::InvalidName), "name {byte:#04x}");
        }
    }
}

#[test]
fn every_byte_of_a_set_cookie_value() {
    for byte in 0..=u8::MAX {
        let value = character(byte).to_string();
        let Ok(field) = set_cookie("a", &value, &plain("/")) else {
            panic!("value {byte:#04x} failed");
        };
        // CK-8.2: a cookie-octet other than `%` is itself, and every other byte of the UTF-8 form is `%XX`.
        let written: String = if is_cookie_octet(byte) && byte != b'%' {
            value.clone()
        } else {
            value.bytes().map(|byte| format!("%{byte:02X}")).collect()
        };
        assert_eq!(field, format!("a={written}; Path=/").into_bytes(), "value {byte:#04x}");
        // CK-8.3: the written value reads back.
        assert_eq!(parse_cookie(&[field]).first(), Some(&("a".to_owned(), value)), "value {byte:#04x}");
    }
}

#[test]
fn every_byte_of_a_set_cookie_path() {
    for byte in 0..=u8::MAX {
        let path = format!("/{}", character(byte));
        let result = set_cookie("a", "b", &plain(&path));
        // CK-10.1 and CK-10.2: a path character is 0x20 to 0x7E other than `;`.
        if (0x20..=0x7E).contains(&byte) && byte != b';' {
            assert_eq!(result, Ok(format!("a=b; Path={path}").into_bytes()), "path byte {byte:#04x}");
        } else {
            assert_eq!(result, Err(Error::InvalidCookiePath), "path byte {byte:#04x}");
        }
    }
}

#[test]
fn the_sizes_of_a_path() {
    // CK-10.1: at most 1024 characters.
    let longest = format!("/{}", "p".repeat(MAX_PATH - 1));
    assert!(set_cookie("a", "b", &plain(&longest)).is_ok());
    let longer = format!("{longest}p");
    assert_eq!(set_cookie("a", "b", &plain(&longer)), Err(Error::InvalidCookiePath));
    assert_eq!(set_cookie("a", "b", &plain("/")), Ok(b"a=b; Path=/".to_vec()));
}

#[test]
fn the_sizes_of_a_name_and_a_value() {
    // CK-11.3: the name and the written value count together, and a name counts as much as a value.
    let name = "n".repeat(MAX_NAME_AND_VALUE - 1);
    assert!(set_cookie(&name, "v", &plain("/")).is_ok());
    assert_eq!(set_cookie(&name, "vv", &plain("/")), Err(Error::TooLarge));
    let name = "n".repeat(MAX_NAME_AND_VALUE);
    assert!(set_cookie(&name, "", &plain("/")).is_ok());
    assert_eq!(set_cookie(&format!("{name}n"), "", &plain("/")), Err(Error::TooLarge));
    // CK-11.3 and CK-8.2: an encoded byte counts as three.
    assert!(set_cookie("a", &"\u{7f}".repeat(1365), &plain("/")).is_ok());
    assert_eq!(set_cookie("a", &"\u{7f}".repeat(1366), &plain("/")), Err(Error::TooLarge));
}

#[test]
fn the_largest_max_age() {
    // CK-11.1: the core takes every u64 and writes it in decimal.
    let attributes = Attributes { max_age: Some(u64::MAX), ..plain("/") };
    assert_eq!(set_cookie("a", "b", &attributes), Ok(b"a=b; Max-Age=18446744073709551615; Path=/".to_vec()));
}

#[test]
fn every_attribute_together() {
    // CK-9.1: the order of the alternatives of cookie-av.
    let attributes = Attributes {
        path: "/".to_owned(),
        max_age: Some(0),
        secure: true,
        http_only: true,
        same_site: Some(SameSite::None),
    };
    assert_eq!(
        set_cookie("__Host-a", "", &attributes),
        Ok(b"__Host-a=; Max-Age=0; Path=/; Secure; HttpOnly; SameSite=None".to_vec())
    );
}

#[test]
fn every_case_of_the_prefixes() {
    // CK-12.1 to CK-12.3: every mix of ASCII case of a prefix is the prefix.
    for prefix in ["__Secure-", "__Host-"] {
        let letters: Vec<usize> =
            prefix.char_indices().filter(|(_, character)| character.is_ascii_alphabetic()).map(|(at, _)| at).collect();
        for mask in 0..(1_u32 << letters.len()) {
            let name: String = prefix
                .char_indices()
                .map(|(at, character)| match letters.iter().position(|letter| *letter == at) {
                    Some(bit) if mask & (1 << bit) != 0 => character.to_ascii_uppercase(),
                    _ => character.to_ascii_lowercase(),
                })
                .collect::<String>()
                + "x";
            assert_eq!(set_cookie(&name, "v", &plain("/")), Err(Error::PrefixNotMet), "{name}");
            let secure = Attributes { secure: true, ..plain("/") };
            assert!(set_cookie(&name, "v", &secure).is_ok(), "{name}");
        }
    }
}
