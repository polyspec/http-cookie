use crate::Error;

/// The most bytes of a name and its written value together (CK-11.3).
pub const MAX_NAME_AND_VALUE: u64 = 4096;

/// The most characters of a path (CK-10.1).
pub const MAX_PATH: u64 = 1024;

/// Whether a length is at most a limit. The limits are `u64`, the type of the constants of the interface contract; a
/// length that `u64` cannot hold is over every limit (CK-10.1, CK-11.3).
fn within(length: usize, limit: u64) -> bool {
    match u64::try_from(length) {
        Ok(length) => length <= limit,
        Err(_) => false,
    }
}

/// The value of the attribute `SameSite` (CK-6.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl SameSite {
    /// The attribute value as `SameSite=<value>` writes it (CK-9.1).
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Strict => "Strict",
            Self::Lax => "Lax",
            Self::None => "None",
        }
    }
}

/// The attributes of a cookie (CK-6.2). `path`, `secure` and `http_only` have no default (CK-1.3); an absent `max_age`
/// or `same_site` writes no attribute (CK-6.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attributes {
    pub path: String,
    pub max_age: Option<u64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<SameSite>,
}

/// The field value of one `Set-Cookie` field line (CK-6.1), ASCII bytes.
///
/// # Errors
///
/// The checks run in the order of CK-6.4 and the first that fails is the error: `InvalidName` (CK-7.2),
/// `InvalidCookiePath` (CK-10.2), `TooLarge` (CK-11.3), `PrefixNotMet` (CK-12.3) and `SameSiteNoneNotSecure`
/// (CK-12.4).
pub fn set_cookie(name: &str, value: &str, attributes: &Attributes) -> Result<Vec<u8>, Error> {
    if !is_token(name) {
        return Err(Error::InvalidName);
    }
    if !is_path(&attributes.path) {
        return Err(Error::InvalidCookiePath);
    }
    let written = encode_value(value);
    if !within(name.len().saturating_add(written.len()), MAX_NAME_AND_VALUE) {
        return Err(Error::TooLarge);
    }
    if !meets_prefix(name, attributes) {
        return Err(Error::PrefixNotMet);
    }
    if attributes.same_site == Some(SameSite::None) && !attributes.secure {
        return Err(Error::SameSiteNoneNotSecure);
    }
    // CK-9.1: the order of the alternatives of `cookie-av`.
    let mut field = Vec::with_capacity(name.len() + written.len() + attributes.path.len() + 64);
    field.extend_from_slice(name.as_bytes());
    field.push(b'=');
    field.extend_from_slice(&written);
    if let Some(max_age) = attributes.max_age {
        // CK-11.1: decimal without leading zeros.
        field.extend_from_slice(b"; Max-Age=");
        field.extend_from_slice(max_age.to_string().as_bytes());
    }
    field.extend_from_slice(b"; Path=");
    field.extend_from_slice(attributes.path.as_bytes());
    if attributes.secure {
        field.extend_from_slice(b"; Secure");
    }
    if attributes.http_only {
        field.extend_from_slice(b"; HttpOnly");
    }
    if let Some(same_site) = attributes.same_site {
        field.extend_from_slice(b"; SameSite=");
        field.extend_from_slice(same_site.as_str().as_bytes());
    }
    Ok(field)
}

/// Whether the name is a `token` (CK-7.1).
fn is_token(name: &str) -> bool {
    !name.is_empty() && name.bytes().all(is_tchar)
}

/// Whether the byte is a `tchar` of RFC 9110, section 5.6.2 (CK-7.1).
const fn is_tchar(byte: u8) -> bool {
    matches!(
        byte,
        b'!' | b'#'
            | b'$'
            | b'%'
            | b'&'
            | b'\''
            | b'*'
            | b'+'
            | b'-'
            | b'.'
            | b'^'
            | b'_'
            | b'`'
            | b'|'
            | b'~'
            | b'0'..=b'9'
            | b'a'..=b'z'
            | b'A'..=b'Z'
    )
}

/// Whether the path starts with `/`, has only the characters 0x20 to 0x7E other than `;` and at most 1024 of them
/// (CK-10.1).
fn is_path(path: &str) -> bool {
    path.starts_with('/')
        && within(path.len(), MAX_PATH)
        && path.bytes().all(|byte| matches!(byte, 0x20..=0x7E) && byte != b';')
}

/// Whether the byte is a `cookie-octet` of RFC 6265, section 4.1.1 (CK-8.2).
const fn is_cookie_octet(byte: u8) -> bool {
    matches!(byte, 0x21 | 0x23..=0x2B | 0x2D..=0x3A | 0x3C..=0x5B | 0x5D..=0x7E)
}

/// The value as it is written (CK-8.2): its UTF-8 bytes, with `%` and every byte that is not a `cookie-octet` written
/// as `%` and two uppercase hexadecimal digits.
fn encode_value(value: &str) -> Vec<u8> {
    let mut written = Vec::with_capacity(value.len());
    for byte in value.bytes() {
        if is_cookie_octet(byte) && byte != b'%' {
            written.push(byte);
        } else {
            written.extend_from_slice(&[b'%', upper_hex(byte >> 4), upper_hex(byte & 0x0F)]);
        }
    }
    written
}

/// The uppercase hexadecimal digit of a value below 16 (CK-8.2).
const fn upper_hex(nibble: u8) -> u8 {
    if nibble < 10 {
        b'0' + nibble
    } else {
        b'A' + nibble - 10
    }
}

/// Whether the attributes meet the condition of the prefix of the name, compared without regard to ASCII case
/// (CK-12.1, CK-12.2). The library writes no `Domain` (CK-9.3), so the condition of `__Host-` on it always holds.
fn meets_prefix(name: &str, attributes: &Attributes) -> bool {
    if has_prefix(name, "__host-") {
        return attributes.secure && attributes.path == "/";
    }
    if has_prefix(name, "__secure-") {
        return attributes.secure;
    }
    true
}

/// Whether the name starts with the prefix, compared without regard to ASCII case (CK-12.3).
fn has_prefix(name: &str, prefix: &str) -> bool {
    name.as_bytes().get(..prefix.len()).is_some_and(|start| start.eq_ignore_ascii_case(prefix.as_bytes()))
}
