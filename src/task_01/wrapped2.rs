use std::fmt;
use std::fmt::Display;

/// Ordinal(T) wraps a value to be represented as an ordinal number.
///
/// Since inner value is private, and provided implementations are just
/// for a selected number of types, incl. i32, i64, u32, u64 and others
/// it could be used only with numeric types that are Integers
///
/// Example:
///
/// ```rust
/// let x = 1.try_into_ordinal().unwrap(); // is Ordinal(1)
/// println!("x: {}", x); // prints `x: 1st`
/// ```
///
/// This one is better in my opinion because it allows you to be sure that inner value
/// is in a half-closed intercval from 1 to infiniti, i.e. [1..). Also it's enforced that
/// the inner value is integer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ordinal<T>(T);

/// This trait is created to replace the `TryInto` trait of the std library.
/// `TryFrom` uses `Into` that uses `From`, which cannot be applied to the current idea.
pub trait TryIntoOrdinal<T> {
    type Error;
    fn try_into_ordinal(self) -> Result<Ordinal<T>, Self::Error>;
}

/// The following macro implements `TryIntoOrdinal<T>` for `U` where `T` is the same type as `U`
///
/// Providing this method as the only way to create an Ordinal type allows us to make sure
/// the inner value is Integer and is greater than zero
macro_rules! impl_try_into_ordinal {
    ($typ:ident) => {
        impl TryIntoOrdinal<$typ> for $typ {
            type Error = &'static str;

            fn try_into_ordinal(self) -> Result<Ordinal<Self>, Self::Error> {
                if self <= 0 {
                    Err("Ordinal inner value must be greater than zero")
                } else {
                    Ok(Ordinal(self))
                }
            }
        }
    };
}

impl_try_into_ordinal!(i8);
impl_try_into_ordinal!(i16);
impl_try_into_ordinal!(i32);
impl_try_into_ordinal!(i64);

impl_try_into_ordinal!(u8);
impl_try_into_ordinal!(u16);
impl_try_into_ordinal!(u32);
impl_try_into_ordinal!(u64);

// more implementations (e.g. for u128 and i128) could be added with conditional compilation

impl<T> Display for Ordinal<T>
where
    T: Display + num::Integer,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.to_string();

        // The following code assumes that the inner value is integer and greater than zero
        //
        // Fortunately, with this implementation it's impossible to initialize a struct with
        // a negative number
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

#[derive(Clone, Debug)]
pub enum OrdinalError {
    ConvertError,
}

/// End-user function
///
/// Returns an ordinal representation of the input integer as a String
///
/// Example usage:
///
/// ```rust
/// println!("ordinal 1: {}", ordinal(1).unwrap()); // prints "ordinal 1: 1st"
/// ```
pub fn ordinal<T>(input: T) -> Result<String, OrdinalError>
where
    T: TryIntoOrdinal<T, Error = &'static str> + Display + num::Integer,
{
    let result = input.try_into_ordinal();
    match result {
        Ok(ordinal) => Ok(ordinal.to_string()),
        Err(_e) => {
            // maybe log an error, maybe return as is, depends on the needs
            Err(OrdinalError::ConvertError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types() {
        assert_eq!(Ok(Ordinal(1)), (1 as i8).try_into_ordinal());
        assert_eq!(Ok(Ordinal(1)), (1 as i16).try_into_ordinal());
        assert_eq!(Ok(Ordinal(1)), (1 as i32).try_into_ordinal());
        assert_eq!(Ok(Ordinal(1)), (1 as i64).try_into_ordinal());
        assert_eq!(Ok(Ordinal(1)), (1 as u8).try_into_ordinal());
        assert_eq!(Ok(Ordinal(1)), (1 as u16).try_into_ordinal());
        assert_eq!(Ok(Ordinal(1)), (1 as u32).try_into_ordinal());
        assert_eq!(Ok(Ordinal(1)), (1 as u64).try_into_ordinal());
    }

    #[test]
    fn various() {
        assert_eq!(Ok(Ordinal(1)), 1.try_into_ordinal());
        assert_eq!(Ordinal(1), 1.try_into_ordinal().unwrap());
        assert_eq!("1st", 1.try_into_ordinal().unwrap().to_string());
    }

    #[test]
    fn try_from_to_ok() {
        let test_cases = vec![
            (Ordinal(1), 1),
            (Ordinal(2), 2),
            (Ordinal(3), 3),
            (Ordinal(4), 4),
            (Ordinal(11), 11),
            (Ordinal(12), 12),
            (Ordinal(21), 21),
        ];

        for (expected, input) in test_cases {
            assert_eq!(expected, input.try_into_ordinal().unwrap())
        }
    }

    #[test]
    fn try_from_to_err() {
        let test_cases = vec![-11, -10, -3, -2, -1, 0];

        for input in test_cases {
            assert!(input.try_into_ordinal().is_err())
        }
    }

    #[test]
    fn converted() {
        let test_cases = vec![
            ("1st", 1),
            ("2nd", 2),
            ("3rd", 3),
            ("4th", 4),
            ("11th", 11),
            ("12th", 12),
            ("21st", 21),
        ];

        for (expected, input) in test_cases {
            assert_eq!(expected, input.try_into_ordinal().unwrap().to_string())
        }
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
        ];

        for (expected, input) in test_cases {
            assert_eq!(expected, ordinal(input).unwrap());
        }
    }

    #[test]
    fn ordinals_err() {
        let test_cases = vec![-11, -10, -3, -2, -1, 0];

        for input in test_cases {
            assert!(ordinal(input).is_err());
        }
    }
}
