use crate::task_03::{Obfuscatable, Obfuscated};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// This is a simplified representation of the email address, but it's enough for the purposes
/// of this task
pub struct Email {
    local: String,
    domain: String,
}

/// This is not a truly correct parser for an email.
///
/// It's not that easy to parse an email address. One tries to parse it with regexes. Although,
/// it could be a decent solution, it won't cover the Internet Message Format RFCs.
///
/// Hence, I won't validate an address here, just parse it to easy the life
impl FromStr for Email {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('@').collect();

        if parts.len() != 2 {
            return Err("not an email".into());
        }

        Ok(Email {
            local: parts[0].into(),
            domain: parts[1].into(),
        })
    }
}

impl Obfuscatable for Email {}

impl Display for Obfuscated<Email> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let chars = self.0.local.chars();
        if let Some(c) = chars.clone().next() {
            write!(f, "{}", c)?;
        }

        let len = chars.clone().count();

        if len > 2 {
            write!(f, "*****")?;
        }

        if len > 1 {
            if let Some(c) = chars.last() {
                write!(f, "{}", c)?;
            }
        }

        write!(f, "@{}", self.0.domain)
    }
}
