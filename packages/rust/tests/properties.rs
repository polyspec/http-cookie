//! Properties for random input (CK-14.4).

use polyspec_http_cookie::{parse_cookie, set_cookie, Attributes, Error, SameSite};
use proptest::prelude::*;

/// Bytes that the grammar gives a meaning, more often than their share of random bytes.
fn line() -> impl Strategy<Value = Vec<u8>> {
    let byte = prop_oneof![
        3 => prop::sample::select(b"; \t=%aA0fF+\"".to_vec()),
        1 => any::<u8>(),
    ];
    prop::collection::vec(byte, 0..40)
}

fn token() -> impl Strategy<Value = String> {
    "[!#$%&'*+.^_`|~0-9A-Za-z-]{1,20}"
}

fn same_site() -> impl Strategy<Value = Option<SameSite>> {
    prop_oneof![Just(None), Just(Some(SameSite::Strict)), Just(Some(SameSite::Lax)), Just(Some(SameSite::None))]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2000))]

    /// CK-2.3: the lines give the cookies of the lines joined with `; `. CK-5.4: reading never panics.
    #[test]
    fn lines_read_as_joined(lines in prop::collection::vec(line(), 0..5)) {
        let joined = lines.join(&b"; "[..]);
        prop_assert_eq!(parse_cookie(&lines), parse_cookie(&[joined]));
    }

    /// CK-5.1, CK-5.2 and CK-4.5: no name twice and no empty name.
    #[test]
    fn names_are_unique(lines in prop::collection::vec(line(), 0..5)) {
        let cookies = parse_cookie(&lines);
        let mut names: Vec<&String> = cookies.iter().map(|(name, _)| name).collect();
        names.sort();
        names.dedup();
        prop_assert_eq!(names.len(), cookies.len());
        for (name, _) in &cookies {
            prop_assert!(!name.is_empty());
            // CK-3.2: the spaces and tabs at the start of a pair are removed; those before `=` are bytes of the name.
            prop_assert!(!name.starts_with([' ', '\t']));
        }
    }

    /// CK-8.3: reading a written cookie gives its name and value; CK-6.1: the field value is ASCII; CK-8.2: the
    /// written value has only cookie-octets.
    #[test]
    fn a_written_cookie_reads_back(
        name in token(),
        value in any::<String>(),
        path in "/[ -:<-~]{0,30}",
        max_age in proptest::option::of(any::<u64>()),
        secure in any::<bool>(),
        http_only in any::<bool>(),
        same_site in same_site(),
    ) {
        let attributes = Attributes { path, max_age, secure, http_only, same_site };
        match set_cookie(&name, &value, &attributes) {
            Ok(field) => {
                prop_assert!(field.is_ascii());
                let written = &field[name.len() + 1..];
                let Some(end) = written.windows(2).position(|window| window == b"; ") else {
                    return Err(TestCaseError::fail("the field has no attribute after the value"));
                };
                let written = &written[..end];
                prop_assert!(written.iter().all(|byte| matches!(byte, 0x21 | 0x23..=0x2B | 0x2D..=0x3A | 0x3C..=0x5B | 0x5D..=0x7E)));
                let read = parse_cookie(std::slice::from_ref(&field));
                prop_assert_eq!(read.first(), Some(&(name, value)));
            }
            Err(Error::PrefixNotMet) => {
                let lower = name.to_ascii_lowercase();
                prop_assert!(lower.starts_with("__secure-") || lower.starts_with("__host-"));
            }
            Err(Error::SameSiteNoneNotSecure) => prop_assert!(same_site == Some(SameSite::None) && !secure),
            Err(Error::TooLarge) => prop_assert!(name.len() + value.len() > 4096 / 3),
            Err(error) => prop_assert!(false, "unexpected failure {}", error.kind()),
        }
    }
}
