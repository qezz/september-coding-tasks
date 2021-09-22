use std::fmt;
use std::fmt::Display;

use num;

/// Wrapper to cover ordinal numbers
///
/// This wrapper assumes that the inner value could be converted into ordinal number.
///
/// The corner cases (which are 0 and negative numbers) are not covered.
/// Hence, for 0 it returns "0th", the negatives are treated like positive numbers.
///
/// Also, this could be extended to work with BigInt types.
#[derive(Copy, Clone, Debug)] // Probably worth it to add more std derivations
pub struct Ordinal<T: num::Integer>(pub T);

/// This trait is just to show that it is possible to create constructions like
///
/// ```rust
/// let x = 1.ordinal().to_string();
/// ```
///
/// to get an ordinal value.
/// Since this never fails, it is similar to simple wrapping without checks.
pub trait IntoOrdinal {
    fn into_ordinal(self) -> Ordinal<Self>
    where
        Self: num::Integer,
    {
        Ordinal(self)
    }
}

impl IntoOrdinal for i32 {}

impl<T> Display for Ordinal<T>
where
    T: Display + num::Integer,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.to_string();

        // I would rather use some kind of compile time checks te express Natural numbers
        // e.g. x is Integer && x is greater than 1
        // But these kinds of checks are not available in rust, (yet?)
        //
        // I add the `num::Integer` constraint to the generic type `T` to make sure the inner
        // value is a number. `num::Integer` is implemented for all primitive integer types
        //
        // I saw the similar implementation on the Internet some time ago, though it allows
        // the values of 0 and negatives
        let suffix = if s.ends_with("1") && !s.ends_with("11") {
            "st"
        } else if s.ends_with("2") && !s.ends_with("12") {
            "nd"
        } else if s.ends_with("3") && !s.ends_with("13") {
            "rd"
        } else {
            "th"
        };
        write!(f, "{}{}", s, suffix)
    }
}

/// Returns an ordinal representation of the input integer as a String
///
/// Example usage:
///
/// ```rust
/// println!("ordinal 1: {}", ordinal(1)); // prints "ordinal 1: 1st"
/// ```
pub fn ordinal<T: IntoOrdinal + num::Integer + Display>(input: T) -> String {
    input.into_ordinal().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!("1st", Ordinal(1).to_string())
    }

    #[test]
    fn second() {
        assert_eq!("2nd", Ordinal(2).to_string())
    }

    #[test]
    fn minus1() {
        assert_eq!("-1st", Ordinal(-1).to_string())
    }

    #[test]
    fn zero() {
        assert_eq!("0th", Ordinal(0).to_string())
    }

    #[test]
    fn first_trait() {
        assert_eq!("1st", 1.into_ordinal().to_string())
    }

    #[test]
    fn ordinals() {
        let test_cases = vec![
            ("1st", 1),
            ("2nd", 2),
            ("3rd", 3),
            ("4th", 4),
            ("11th", 11),
            ("12th", 12),
            ("21st", 21),
            ("0th", 0),
            ("-1st", -1),
            ("-2nd", -2),
        ];

        for (expected, input) in test_cases {
            assert_eq!(expected, ordinal(input));
        }
    }
}
