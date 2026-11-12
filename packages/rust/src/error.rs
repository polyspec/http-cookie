use std::fmt;

/// A failure of `set_cookie` (CK-13). Every variant has a kind, no HTTP status (CK-13.3) and the message that its
/// rule states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// The name is not a token (CK-7.2).
    InvalidName,
    /// The path is not an absolute path of at most 1024 printable ASCII characters without `;` (CK-10.2).
    InvalidCookiePath,
    /// The name and the written value are longer than 4096 bytes together (CK-11.3).
    TooLarge,
    /// A name with the prefix `__Secure-` or `__Host-` lacks the attributes that the prefix requires (CK-12.3).
    PrefixNotMet,
    /// `SameSite=None` without `Secure` (CK-12.4).
    SameSiteNoneNotSecure,
}

impl Error {
    /// The kind of the failure, which does not change (CK-13.2).
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::InvalidName => "invalid_name",
            Self::InvalidCookiePath => "invalid_cookie_path",
            Self::TooLarge => "too_large",
            Self::PrefixNotMet => "prefix_not_met",
            Self::SameSiteNoneNotSecure => "same_site_none_not_secure",
        }
    }

    /// The HTTP status of the failure: none, because a failure is an error of the caller's arguments and not of a
    /// request (CK-13.3).
    #[must_use]
    pub const fn status(&self) -> Option<u16> {
        None
    }

    /// The message that the rule of the failure states (CK-13.1).
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::InvalidName => "a cookie name is not a token (CK-7)",
            Self::InvalidCookiePath => {
                "a cookie path is not an absolute path of at most 1024 printable ASCII characters without ';' (CK-10)"
            }
            Self::TooLarge => "a cookie name and value are longer than 4096 bytes (CK-11)",
            Self::PrefixNotMet => {
                "a cookie name with the prefix __Secure- or __Host- lacks the attributes that the prefix requires (CK-12)"
            }
            Self::SameSiteNoneNotSecure => "a cookie with SameSite=None lacks Secure (CK-12)",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message())
    }
}

impl std::error::Error for Error {}
