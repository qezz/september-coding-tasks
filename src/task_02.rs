use chrono::format::ParseError;
use chrono::{Datelike, NaiveDate, Weekday};

/// To be honest, number of Sundays could be calculated just using the week-of-the-year number,
/// but I decided to generalize it a bit, just to be sure that it is easy to modify the day
/// the week.

pub struct WeekdaysCounter {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl WeekdaysCounter {
    fn new(start_date: NaiveDate, end_date: NaiveDate) -> Self {
        Self {
            start_date,
            end_date,
        }
    }

    /// A bit weird way to count the dates, but it does the job.
    ///
    /// The idea is to count a number of 'full weeks' that fit into the timeframe starting with
    /// the target weekday.
    fn count(&self, day_of_week: Weekday) -> u32 {
        let (year_day_from, year_day_to) = (self.start_date.ordinal(), self.end_date.ordinal());
        if year_day_to < year_day_from {
            return 0;
        }

        // total number of days in a timeframe
        let num_days = year_day_to - year_day_from;

        // trying to calculate the offset between the `start_date` and the next weekday.
        let sign_start_diff: i32 = day_of_week.num_days_from_monday() as i32
            - self.start_date.weekday().num_days_from_monday() as i32;

        // if this fits this week, the diff is a positive number up to 6
        // (counting weekdays from 0 to 6, or from 1 to 7). Otherwise, it is negative,
        // hence adding it up to 7 will give us the offset.
        let start_offset = if sign_start_diff >= 0 {
            sign_start_diff
        } else {
            7 + sign_start_diff
        };

        // sometimes the offset is out of the date range
        if (num_days as i32) < start_offset {
            return 0;
        }

        // `+1` is needed since we are counting with the last day included
        (num_days - start_offset as u32) / 7 + 1
    }
}

/// Returns a number of Sundays in the provided date range
///
/// The range is inclusive on both sides
pub fn count_sundays((date_from, date_to): (&str, &str)) -> Result<u32, ParseError> {
    let format = "%d-%m-%Y";
    let start_date = NaiveDate::parse_from_str(date_from, format)?;
    let end_date = NaiveDate::parse_from_str(date_to, format)?;

    Ok(WeekdaysCounter::new(start_date, end_date).count(Weekday::Sun))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(5, count_sundays(("01-05-2021", "30-05-2021")).unwrap());
    }

    #[test]
    fn days1() {
        let test_cases = vec![
            (4, Weekday::Mon),
            (4, Weekday::Tue),
            (4, Weekday::Wed),
            (4, Weekday::Thu),
            (4, Weekday::Fri),
            (5, Weekday::Sat),
            (5, Weekday::Sun),
        ];

        let format = "%d-%m-%Y";
        let start_date = NaiveDate::parse_from_str("01-05-2021", format).unwrap();
        let end_date = NaiveDate::parse_from_str("30-05-2021", format).unwrap();

        for (expected, weekday) in test_cases {
            assert_eq!(
                expected,
                WeekdaysCounter::new(start_date, end_date).count(weekday)
            );
        }
    }

    #[test]
    fn days2() {
        let test_cases = vec![
            (0, Weekday::Mon),
            (0, Weekday::Tue),
            (0, Weekday::Wed),
            (0, Weekday::Thu),
            (0, Weekday::Fri),
            (1, Weekday::Sat),
            (0, Weekday::Sun),
        ];

        let format = "%d-%m-%Y";
        let start_date = NaiveDate::parse_from_str("01-05-2021", format).unwrap();
        let end_date = NaiveDate::parse_from_str("01-05-2021", format).unwrap();

        for (expected, weekday) in test_cases {
            assert_eq!(
                expected,
                WeekdaysCounter::new(start_date, end_date).count(weekday)
            );
        }
    }

    #[test]
    fn days3() {
        let test_cases = vec![
            (1, Weekday::Mon),
            (1, Weekday::Tue),
            (1, Weekday::Wed),
            (1, Weekday::Thu),
            (1, Weekday::Fri),
            (1, Weekday::Sat),
            (1, Weekday::Sun),
        ];

        let format = "%d-%m-%Y";
        let start_date = NaiveDate::parse_from_str("01-05-2021", format).unwrap();
        let end_date = NaiveDate::parse_from_str("07-05-2021", format).unwrap();

        for (expected, weekday) in test_cases {
            assert_eq!(
                expected,
                WeekdaysCounter::new(start_date, end_date).count(weekday)
            );
        }
    }

    #[test]
    fn days4() {
        let test_cases = vec![
            (0, Weekday::Mon),
            (0, Weekday::Tue),
            (0, Weekday::Wed),
            (0, Weekday::Thu),
            (0, Weekday::Fri),
            (0, Weekday::Sat),
            (0, Weekday::Sun),
        ];

        let format = "%d-%m-%Y";
        let start_date = NaiveDate::parse_from_str("02-05-2021", format).unwrap();
        let end_date = NaiveDate::parse_from_str("01-05-2021", format).unwrap();

        for (expected, weekday) in test_cases {
            assert_eq!(
                expected,
                WeekdaysCounter::new(start_date, end_date).count(weekday)
            );
        }
    }

    #[test]
    fn days5() {
        let test_cases = vec![
            (2, Weekday::Mon),
            (2, Weekday::Tue),
            (2, Weekday::Wed),
            (2, Weekday::Thu),
            (1, Weekday::Fri),
            (2, Weekday::Sat),
            (2, Weekday::Sun),
        ];

        let format = "%d-%m-%Y";
        let start_date = NaiveDate::parse_from_str("01-05-2021", format).unwrap();
        let end_date = NaiveDate::parse_from_str("13-05-2021", format).unwrap();

        for (expected, weekday) in test_cases {
            assert_eq!(
                expected,
                WeekdaysCounter::new(start_date, end_date).count(weekday)
            );
        }
    }
}
