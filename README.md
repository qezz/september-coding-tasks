# September Coding Tasks

A set of tasks for NDA company.

## Tasks description

1. Write a function that takes an Integer and returns it as a string with the correct ordinal indicator suffix (in English). 
   * Examples: `1` => `1st`, `2` => `2nd`.
2. Write a function that takes two dates (date_from, date_to, in dd-mm-yyyy format) and returns the number of Sundays in that range. 
   * Example: `(‘01-05-2021’, ‘30-05-2021’)` => `5`.
3. Mask personal information: create a function that takes a String as input and returns it partly obfuscated. 
  The function only recognizes emails and phone numbers, any other String that doesn’t match these types results in an error.
   * Emails: emails need to be in a valid email format. To obfuscate it, it should be converted to lowercase and all characters 
     in the local-part between the first and last should be replaced by 5 asterisks (*). 
      * Example: `local-part@domain-name.com` => `l*****t@domain-name.com`.
   * Phone numbers: a phone number consists of at least 9 digits (0-9) and may contain these two characters (‘ ‘, ‘+’) 
     where ‘+’ is only accepted when is the first character. To obfuscate it, spaces (‘ ‘) are converted to dashes (‘-’), 
     any digit is converted to an asterisk (‘*’) except for the last 4, which remain unchanged and the plus sign (‘+’) 
     also remains unchanged (if present). 
      * Example: `+44 123 456 789` => `+**-***-**6-789`.

## Solutions

### Ordinal

I provide two solutions, one is very simple, the second is more complicated
to provide flexibility.

The implementation for BigInt types is not included, though it could be easily 
added later.

* `task_01/simple` - simple solution, but allows negative numbers and zero.
  ```rust
  assert_eq!("1st", 1.into_ordinal().to_string());
  assert_eq!("0th", 0.into_ordinal().to_string());
  assert_eq!("-1st", (-1).into_ordinal().to_string());
  ```
  
  Function:
  
  ```rust
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
  ```
* `task_01/wrapped` - more complicated solution, the inner value is limited 
  to integer values that are greater than 0
  ```rust
  assert_eq!(Ok(Ordinal(1)), Ordinal::try_from(1)); // result is Ok
  assert!(Ordinal::try_from(0).is_err()); // result is Error
  assert!(Ordinal::try_from(-1).is_err()); // result is Error
  ```
  
  Trait `Display` (`.to_string()`) is implemented the same way.
  
  Though, there is a major problem with `try_into()`, it requires 
  the implementation of `Into` for the inner type. Due to this limitation
  it's not easy to write a function `fn ordinal(T)`, where `T` is type constrained
  
  For more information see:
  * https://github.com/rust-lang/rust/pull/51564
  * https://stackoverflow.com/questions/50437732/how-do-i-convert-a-usize-to-a-u32-using-tryfrom
* `task_01/wrapped2` - another attempt with this approach. I use `.try_into_ordinal()` method 
  to be able to specify constraints easily.
  
  Function:
  
  ```rust
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
  ``` 

### Dates, number of Sundays

I decided to implement a function that can take a weekday as a parameter

I think it is possible to simplify the solution if the actual task is to
count a number of weeks included into the date range. This could be achieved
using a number that represents a week from the start of the year. Sure, 
some corner cases should be covered.

Back to the solution, 

Final function to count Sundays is:

```rust
/// Returns a number of Sundays in the provided date range
///
/// The range is inclusive on both sides
pub fn count_sundays((date_from, date_to): (&str, &str)) -> Result<u32, ParseError> {
    let format = "%d-%m-%Y";
    let start_date = NaiveDate::parse_from_str(date_from, format)?;
    let end_date = NaiveDate::parse_from_str(date_to, format)?;

    Ok(WeekdaysCounter::new(start_date, end_date).count(Weekday::Sun))
}
```

But you can replace the `Weekday::Sun` with any weekday you like.

### Obfuscate

Disclaimer: I didn't try to create the best email address parses and 
phone number parser, hence I use simple assumptions about emails and 
phone numbers.

The final function is: 

```rust
/// Obfuscate the input
///
/// Accepts an email or a phone number as an input. If input couldn't be parsed,
/// returns an error `ObfuscationError::UnknownInput`
///
/// Usage example:
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
```

As you can see, I'm trying to parse the input string as an Email and a PhoneNumber.

The function returns an Error when `parse()` fails.

## Author

Sergey Mishin

## License

MIT