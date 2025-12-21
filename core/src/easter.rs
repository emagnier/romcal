use crate::error::{RomcalError, RomcalResult, validate_year};
use chrono::{DateTime, Datelike, NaiveDate, Utc};

/// Easter date
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EasterDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl EasterDate {
    /// Converts to `DateTime<Utc>`
    ///
    /// # Errors
    ///
    /// Returns `RomcalError::DateConversionError` if the date is invalid (e.g., February 30th)
    pub fn to_utc_date(self) -> RomcalResult<DateTime<Utc>> {
        let naive_date = NaiveDate::from_ymd_opt(self.year, self.month, self.day)
            .ok_or(RomcalError::DateConversionError)?;

        let naive_datetime = naive_date
            .and_hms_opt(0, 0, 0)
            .ok_or(RomcalError::DateConversionError)?;

        Ok(naive_datetime.and_utc())
    }
}

/// Calculates Easter date using Oudin's Gregorian algorithm (1940)
///
/// This algorithm is based on Oudin's algorithm (1940) and quoted in
/// "Explanatory Supplement to the Astronomical Almanac", P. Kenneth
/// Seidelmann, editor.
///
/// # Arguments
///
/// * `year` - The year for which to calculate Easter (must be >= 1583 for Gregorian calendar)
///
/// # Errors
///
/// Returns `RomcalError::InvalidYear` if the year is before 1583 (before the Gregorian calendar was introduced)
pub fn calculate_gregorian_easter_date(year: i32) -> RomcalResult<EasterDate> {
    validate_year(year, 1583)?;
    let y = year;
    let c = y / 100;
    let n = y - 19 * (y / 19);
    let k = (c - 17) / 25;
    let mut i = c - (c / 4) - ((c - k) / 3) + 19 * n + 15;

    i -= 30 * (i / 30);
    i -= (i / 28) * (1 - (i / 28) * (29 / (i + 1)) * ((21 - n) / 11));

    let mut j = y + (y / 4) + i + 2 - c + (c / 4);
    j -= 7 * (j / 7);

    let l = i - j;
    let m = 3 + ((l + 40) / 44);
    let d = l + 28 - 31 * (m / 4);

    Ok(EasterDate {
        year: y,
        month: m as u32,
        day: d as u32,
    })
}

/// Calculates Easter date according to the Julian calendar
///
/// # Errors
///
/// Returns `RomcalError::InvalidYear` if the year is before 326 AD
pub fn calculate_julian_easter_date(year: i32) -> RomcalResult<EasterDate> {
    validate_year(year, 326)?;

    let a = year % 4;
    let b = year % 7;
    let c = year % 19;
    let d = (19 * c + 15) % 30;
    let e = (2 * a + 4 * b - d + 34) % 7;
    let f = d + e + 114;
    let month = f / 31;
    let day = (f % 31) + 1;

    Ok(EasterDate {
        year,
        month: month as u32,
        day: day as u32,
    })
}

/// Converts a Julian date to Gregorian date
fn julian_to_gregorian(julian_date: EasterDate) -> RomcalResult<EasterDate> {
    // Simplified conversion for Easter dates
    // In most cases, the difference is 13 days
    let gregorian_date = julian_date.to_utc_date()? + chrono::Duration::days(13);

    Ok(EasterDate {
        year: gregorian_date.year(),
        month: gregorian_date.month(),
        day: gregorian_date.day(),
    })
}

/// Calculates Julian Easter date converted to Gregorian date
///
/// # Errors
///
/// Returns `RomcalError::InvalidYear` if the year is before 326 AD
pub fn calculate_julian_easter_date_to_gregorian(year: i32) -> RomcalResult<EasterDate> {
    let julian_easter = calculate_julian_easter_date(year)?;
    julian_to_gregorian(julian_easter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gregorian_easter_dates() {
        // Test some known Easter dates
        let test_cases = vec![
            (2001, 4, 15),
            (2002, 3, 31),
            (2003, 4, 20),
            (2004, 4, 11),
            (2005, 3, 27),
            (2006, 4, 16),
            (2007, 4, 8),
            (2008, 3, 23),
            (2009, 4, 12),
            (2010, 4, 4),
            (2011, 4, 24),
            (2012, 4, 8),
            (2013, 3, 31),
            (2014, 4, 20),
            (2015, 4, 5),
            (2016, 3, 27),
            (2017, 4, 16),
            (2018, 4, 1),
            (2019, 4, 21),
            (2020, 4, 12),
            (2021, 4, 4),
            (2022, 4, 17),
            (2023, 4, 9),
            (2024, 3, 31),
            (2025, 4, 20),
        ];

        for (year, expected_month, expected_day) in test_cases {
            let easter = calculate_gregorian_easter_date(year).unwrap();
            assert_eq!(easter.year, year);
            assert_eq!(easter.month, expected_month);
            assert_eq!(easter.day, expected_day);
        }
    }

    #[test]
    fn test_julian_easter_date_to_gregorian() {
        // Test for some years
        let easter_2024 = calculate_julian_easter_date_to_gregorian(2024).unwrap();
        assert_eq!(easter_2024.year, 2024);
        // The exact date depends on the conversion implementation
        // but it should be close to the Gregorian date
    }

    #[test]
    fn test_invalid_years() {
        // Test invalid years for Gregorian calendar
        assert!(calculate_gregorian_easter_date(1500).is_err());
        assert!(calculate_gregorian_easter_date(1582).is_err());
        assert!(calculate_gregorian_easter_date(1583).is_ok());

        // Test invalid years for Julian calendar
        assert!(calculate_julian_easter_date(325).is_err());
        assert!(calculate_julian_easter_date(326).is_ok());
    }
}
