use crate::task_03::{Obfuscatable, Obfuscated};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A simplified representation of phone numbers
pub struct PhoneNumber {
    has_plus_prefix: bool,
    parts: Vec<u64>,
}

/// The same as emails, it is also not easy to parse the numbers. I provide a simple
/// implementation  that doesn't cover a lot of things. But at least this could be easily
/// replaced with a better solution, without breaking anything.
///
/// IMHO for a robust parsing of these values
impl FromStr for PhoneNumber {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_parts: Vec<&str> = s.trim_start_matches(|sub| sub == '+').split(' ').collect();

        let mut parts = Vec::with_capacity(str_parts.len());

        for part in str_parts {
            let a_number: u64 = part.parse()?;
            parts.push(a_number);
        }

        Ok(PhoneNumber {
            has_plus_prefix: s.starts_with('+'),
            parts,
        })
    }
}

impl Obfuscatable for PhoneNumber {}

impl Display for Obfuscated<PhoneNumber> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // I just write the visible digits first on the reversed string.
        // Then, reverse it back.
        let s = self
            .0
            .parts
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("-");

        let number_of_visible = 4;
        let mut visible = 0;
        let mut output = String::with_capacity(s.len());

        for ch in s.chars().rev() {
            if ch.is_digit(10) {
                if visible < number_of_visible {
                    output.push(ch);
                    visible += 1;
                } else {
                    output.push('*');
                }
            } else {
                output.push('-');
            }
        }

        if self.0.has_plus_prefix {
            write!(f, "+")?;
        }

        write!(f, "{}", output.chars().rev().collect::<String>())
    }
}
