use std::collections::HashSet;

/// The cookies of a request (CK-5.1): the names and values of the `Cookie` field lines, in the order in which each name
/// first gives a cookie, with no name twice.
///
/// `lines` are the field values of the `Cookie` field lines of one request, in the order in which they were received,
/// each as the bytes of its field value (CK-2.1); a request without the field gives no line (CK-2.2). The lines are
/// read one after another (CK-2.3). Reading never fails (CK-5.4): a pair that gives no cookie is one of the rows of the
/// table "Inputs that give no cookie" of the specification.
#[must_use]
pub fn parse_cookie(lines: &[Vec<u8>]) -> Vec<(String, String)> {
    let mut cookies: Vec<(String, String)> = Vec::new();
    let mut names: HashSet<String> = HashSet::new();
    for line in lines {
        // CK-2.3: the end of a line ends a pair as `;` does.
        for pair in line.split(|&byte| byte == b';') {
            if let Some(cookie) = cookie_of_pair(trim(pair), &names) {
                names.insert(cookie.0.clone());
                cookies.push(cookie);
            }
        }
    }
    cookies
}

/// The cookie of one trimmed pair, or none for the pairs of CK-4.4 to CK-4.7 and CK-5.2.
fn cookie_of_pair(pair: &[u8], names: &HashSet<String>) -> Option<(String, String)> {
    // CK-3.3: the pair splits at its first `=`.
    let mut parts = pair.splitn(2, |&byte| byte == b'=');
    let name = parts.next()?;
    // CK-4.4: a pair without `=`, the empty pair included, gives no cookie.
    let raw_value = parts.next()?;
    // CK-4.5: a pair with an empty name gives no cookie.
    if name.is_empty() {
        return None;
    }
    // CK-4.6: a pair whose name is not valid UTF-8 gives no cookie. CK-4.1: the name is not decoded.
    let Ok(name) = std::str::from_utf8(name) else {
        return None;
    };
    // CK-5.2: a later pair of a name that gave a cookie gives none.
    if names.contains(name) {
        return None;
    }
    // CK-4.7: a pair whose decoded value is not valid UTF-8 gives no cookie.
    let Ok(value) = String::from_utf8(percent_decode(raw_value)) else {
        return None;
    };
    Some((name.to_owned(), value))
}

/// The pair without the spaces and tabs at both ends (CK-3.2, a row of the table of inputs that give no cookie).
fn trim(mut pair: &[u8]) -> &[u8] {
    while let [b' ' | b'\t', rest @ ..] = pair {
        pair = rest;
    }
    while let [rest @ .., b' ' | b'\t'] = pair {
        pair = rest;
    }
    pair
}

/// The value with every `%` and two hexadecimal digits replaced by the byte of those digits (CK-4.2); every other
/// byte is itself.
fn percent_decode(raw: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::with_capacity(raw.len());
    let mut index = 0;
    while let Some(&byte) = raw.get(index) {
        if byte == b'%' {
            if let (Some(high), Some(low)) = (hex_digit(raw.get(index + 1)), hex_digit(raw.get(index + 2))) {
                decoded.push((high << 4) | low);
                index += 3;
                continue;
            }
        }
        // CK-4.2: a `%` that two hexadecimal digits do not follow, `+` and every other byte are themselves.
        decoded.push(byte);
        index += 1;
    }
    decoded
}

/// The value of a hexadecimal digit of either case (CK-4.2), or none for another byte or the end of the value.
fn hex_digit(byte: Option<&u8>) -> Option<u8> {
    match byte? {
        digit @ b'0'..=b'9' => Some(digit - b'0'),
        letter @ b'a'..=b'f' => Some(letter - b'a' + 10),
        letter @ b'A'..=b'F' => Some(letter - b'A' + 10),
        _ => None,
    }
}
