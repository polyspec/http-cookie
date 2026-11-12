//! Every case of `contracts/cookies.json` (CK-14.1), and the string boundaries that each case gives (CK-14.3): a
//! `parse` case read with its lines joined with `; ` and with a line split at each `;` (CK-2.3), and a `set_cookie`
//! case read back (CK-8.3).

use polyspec_http_cookie::{parse_cookie, set_cookie, Attributes, SameSite};
use serde_json::Value;

const CONTRACT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../contracts/cookies.json"));

fn contract() -> Value {
    match serde_json::from_str(CONTRACT) {
        Ok(value) => value,
        Err(error) => panic!("contracts/cookies.json is not JSON: {error}"),
    }
}

fn group(contract: &Value, name: &str) -> Vec<Value> {
    match contract.get(name) {
        Some(Value::Array(cases)) if !cases.is_empty() => cases.clone(),
        _ => panic!("contracts/cookies.json has no cases of the group {name}"),
    }
}

fn text<'a>(case: &'a Value, key: &str) -> &'a str {
    match case.get(key) {
        Some(Value::String(text)) => text,
        _ => panic!("case {} has no text {key}", case["id"]),
    }
}

/// The bytes of a byte string of a case: each character U+0000 to U+00FF is its byte (CK-14.2).
fn latin1(text: &str) -> Vec<u8> {
    text.chars()
        .map(|character| match u8::try_from(u32::from(character)) {
            Ok(byte) => byte,
            Err(_) => panic!("{text:?} has a character above U+00FF, which is no byte"),
        })
        .collect()
}

fn lines(case: &Value) -> Vec<Vec<u8>> {
    match case.get("lines") {
        Some(Value::Array(lines)) => lines
            .iter()
            .map(|line| match line {
                Value::String(line) => latin1(line),
                _ => panic!("case {} has a line that is not text", case["id"]),
            })
            .collect(),
        _ => panic!("case {} has no lines", case["id"]),
    }
}

fn cookies(case: &Value) -> Vec<(String, String)> {
    match case.get("cookies") {
        Some(Value::Array(pairs)) => pairs
            .iter()
            .map(|pair| match pair.as_array().map(Vec::as_slice) {
                Some([Value::String(name), Value::String(value)]) => (name.clone(), value.clone()),
                _ => panic!("case {} has a cookie that is not a pair of texts", case["id"]),
            })
            .collect(),
        _ => panic!("case {} has no cookies", case["id"]),
    }
}

fn join(lines: &[Vec<u8>]) -> Vec<u8> {
    lines.join(&b"; "[..])
}

#[test]
fn parse_cases() {
    let mut failures = Vec::new();
    let cases = group(&contract(), "parse");
    for case in &cases {
        let id = text(case, "id");
        let lines = lines(case);
        let expected = cookies(case);
        let actual = parse_cookie(&lines);
        if actual != expected {
            failures.push(format!("{id}: {actual:?} is not {expected:?}"));
        }
        // CK-2.3: the lines give the cookies of the lines joined with `; `.
        let joined = parse_cookie(&[join(&lines)]);
        if joined != expected {
            failures.push(format!("{id}, its lines joined: {joined:?} is not {expected:?}"));
        }
        // CK-2.3: the end of a line ends a pair as `;` does, so a line split at a `;` gives the same cookies.
        for (index, line) in lines.iter().enumerate() {
            for (at, _) in line.iter().enumerate().filter(|(_, byte)| **byte == b';') {
                let mut split = lines.clone();
                split.splice(index..=index, [line[..at].to_vec(), line[at + 1..].to_vec()]);
                let actual = parse_cookie(&split);
                if actual != expected {
                    failures.push(format!("{id}, line {index} split at byte {at}: {actual:?} is not {expected:?}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures.join("\n"));
    assert_eq!(cases.len(), 41, "the number of parse cases");
}

fn attributes(case: &Value) -> Attributes {
    let Some(attributes) = case.get("attributes") else {
        panic!("case {} has no attributes", case["id"]);
    };
    let max_age = match &attributes["max_age"] {
        Value::Null => None,
        Value::Number(number) => match number.as_u64() {
            Some(max_age) => Some(max_age),
            None => panic!("case {} has a max_age that is not a non-negative integer", case["id"]),
        },
        _ => panic!("case {} has a max_age that is not a number", case["id"]),
    };
    let same_site = match &attributes["same_site"] {
        Value::Null => None,
        Value::String(value) if value == "Strict" => Some(SameSite::Strict),
        Value::String(value) if value == "Lax" => Some(SameSite::Lax),
        Value::String(value) if value == "None" => Some(SameSite::None),
        _ => panic!("case {} has a same_site of another value", case["id"]),
    };
    let flag = |key: &str| match &attributes[key] {
        Value::Bool(flag) => *flag,
        _ => panic!("case {} has a {key} that is not a boolean", case["id"]),
    };
    Attributes {
        path: text(attributes, "path").to_owned(),
        max_age,
        secure: flag("secure"),
        http_only: flag("http_only"),
        same_site,
    }
}

#[test]
fn set_cookie_cases() {
    let mut failures = Vec::new();
    let cases = group(&contract(), "set_cookie");
    for case in &cases {
        let id = text(case, "id");
        let name = text(case, "name");
        let value = text(case, "value");
        let result = set_cookie(name, value, &attributes(case));
        match (case.get("header"), case.get("raises"), result) {
            (Some(Value::String(header)), None, Ok(field)) => {
                if field != latin1(header) {
                    failures.push(format!("{id}: {:?} is not {header:?}", String::from_utf8_lossy(&field)));
                }
                // CK-6.1: the field value is ASCII.
                if !field.is_ascii() {
                    failures.push(format!("{id}: the field value is not ASCII"));
                }
                // CK-8.3: reading the written cookie gives its name and value.
                let read = parse_cookie(&[field]);
                if read.first() != Some(&(name.to_owned(), value.to_owned())) {
                    failures.push(format!("{id}: the written cookie reads back as {:?}", read.first()));
                }
            }
            (None, Some(raises), Err(error)) => {
                let actual = serde_json::json!({
                    "kind": error.kind(),
                    "status": error.status(),
                    "category": "value",
                    "message": error.to_string(),
                });
                if &actual != raises {
                    failures.push(format!("{id}: raised {actual} and not {raises}"));
                }
            }
            (Some(_), Some(_), _) => failures.push(format!("{id}: a case has both header and raises (CK-14.2)")),
            (_, _, Ok(field)) => {
                failures.push(format!("{id}: wrote {:?} and failed not", String::from_utf8_lossy(&field)))
            }
            (_, _, Err(error)) => failures.push(format!("{id}: failed with {} unexpectedly", error.kind())),
        }
    }
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures.join("\n"));
    assert_eq!(cases.len(), 57, "the number of set_cookie cases");
}
