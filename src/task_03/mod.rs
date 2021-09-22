mod emails;
mod phone_numbers;

use emails::Email;
use phone_numbers::PhoneNumber;

/// I use approach to wrap the value into a wrapper, to obfuscate it later, when `fmt()` is called.
///
/// If we don't provide access to the inner value, it's (almost) impossible to get the value
///
/// Another approach could be used. It is possible to implement a trait on the String,
/// so it is possible to use the following construct
///
/// ```rust
/// "local@domain".obfuscate().unwrap()
/// ```
///
/// but this is also sub-optimal because we'll have to modify this method every time a new type
/// is added.
/// Also, that approach won't eleminate the `.parse::<T>()` methods, since we need to understand
/// how to modify the string to obfuscate it.
struct Obfuscated<T: ?Sized>(T);

trait Obfuscatable {
    fn obfuscated(self) -> Obfuscated<Self>
    where
        Self: Sized,
    {
        Obfuscated(self)
    }
}

#[derive(Debug, Clone)]
pub enum ObfuscationError {
    UnknownInput,
}

/// Obfuscate the input
///
/// Accepts an email or a phone number as an input. If input couldn't be parsed,
/// returns an error `ObfuscationError::UnknownInput`
///
/// Usage exaxple:
///
/// ```rust
/// // a phone number
/// let obfuscated = obfuscate("+44 123 456 789".into()).unwrap();
/// println!("{}", obfuscated); // prints "+**-***-**6-789"
///
/// // an email address
/// let obfuscated = obfuscate("local-part@domain-name.com".into()).unwrap();
/// println!("{}", obfuscated); // prints "l*****t@domain-name.com"
/// ```
pub fn obfuscate(input: String) -> Result<String, ObfuscationError> {
    if let Ok(parsed_email) = input.parse::<Email>() {
        Ok(parsed_email.obfuscated().to_string())
    } else if let Ok(parsed_phone) = input.parse::<PhoneNumber>() {
        Ok(parsed_phone.obfuscated().to_string())
    } else {
        Err(ObfuscationError::UnknownInput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email1() {
        let input = "a@domain.com";
        let expected = "a@domain.com";
        let actual = &(input.parse::<Email>().unwrap().obfuscated().to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn email2() {
        let input = "ab@domain.com";
        let expected = "ab@domain.com";
        let actual = &(input.parse::<Email>().unwrap().obfuscated().to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn email3() {
        let input = "abc@domain.com";
        let expected = "a*****c@domain.com";
        let actual = &(input.parse::<Email>().unwrap().obfuscated().to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn email4() {
        let input = "abcdefghijk@domain.com";
        let expected = "a*****k@domain.com";
        let actual = &(input.parse::<Email>().unwrap().obfuscated().to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn phone1() {
        let input = "+44 123 456 789";
        let expected = "+**-***-**6-789";
        let actual = &(input
            .parse::<PhoneNumber>()
            .unwrap()
            .obfuscated()
            .to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn phone2() {
        let input = "+7 999 123 45 67";
        let expected = "+*-***-***-45-67";
        let actual = &(input
            .parse::<PhoneNumber>()
            .unwrap()
            .obfuscated()
            .to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn obfuscate1() {
        let input = "+44 123 456 789";
        let expected = "+**-***-**6-789";
        let actual = &obfuscate(input.into()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn obfuscate2() {
        let input = "local-part@domain-name.com";
        let expected = "l*****t@domain-name.com";
        let actual = &obfuscate(input.into()).unwrap();
        assert_eq!(expected, actual);
    }
}
