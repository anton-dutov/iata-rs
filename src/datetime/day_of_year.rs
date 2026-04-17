use super::*;

/// Contains a number of some day in a year, which is an integer that ranges from
/// 1 to 366 (inclusively). In some places this data structure is referred to as
/// "ordinal".
///
/// This structure DOES NOT guarantee that it contains
/// a day number valid for any year, since a year may contain one extra day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
pub struct DayOfYear(u16);

impl DayOfYear {
    /// Constructs a new instance of [`DayOfYear`] based off the day number.
    /// Only numbers within range `1..=366` are accepted.
    ///
    /// # Errors
    /// If `day` is not from `1..=366` [`Error::InvalidDayOfYearRange`] is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use iata::datetime::DayOfYear;
    ///
    /// // Ok
    /// assert!(DayOfYear::new(366).is_ok());
    /// assert!(DayOfYear::new(10).is_ok());
    ///
    /// // Not ok
    /// assert!(DayOfYear::new(0).is_err());
    /// assert!(DayOfYear::new(400).is_err());
    /// ```
    pub fn new(day: u16) -> Result<Self, Error> {
        if day == 0 || day > 366 {
            return Err(Error::InvalidDayOfYearRange(day));
        }

        Ok(Self(day))
    }

    /// Returns the day value, wrapped by this struct.
    pub fn ordinal(&self) -> u16 {
        self.0
    }

    /// Converts this date into [`jiff::civil::Date`]. If it turns out that
    /// `year` is not a leap year and the day is 366'th, [`Error::OverflowNotLeapYear`]
    /// is returned.
    pub fn to_date(&self, year: i32) -> Result<Date, Error> {
        if self.0 == 366 && !is_leap_year(year) {
            return Err(Error::OverflowNotLeapYear);
        }

        Date::new(year as i16, 1, 1)
            .and_then(|d| d.checked_add((self.0 as i64 - 1).days()))
            .map_err(|_| Error::OverflowNotLeapYear)
    }

    /// Tries to figure out what year the day stored in `self` belongs to. Equivalent to
    /// `self.to_date_adapt(today, days)`.
    ///
    /// For more information about algorithm and some examples, see
    /// [`Self::to_date_adapt()`].
    pub fn to_date_adapt_year(&self, offset: Offset, days: u8) -> Result<Date, Error> {
        let now = Timestamp::now().to_zoned(TimeZone::fixed(offset));

        self.to_date_adapt(now.date(), days)
    }

    /// Tries to figure out what year the day stored in `self` belongs to. Here are the
    /// techniques used by the function (here "today's date" is whatever is stored in `for_date`):
    ///
    /// 1. If `self.ordinal()` is more than `days` days earlier than today's date,
    ///    `self.ordinal()` is not more than `days` days later than the start of year -- it
    ///    is considered that `self` belongs to the next year.
    /// 2. If `self.ordinal()` is less than `days` days away from year's end, today's date
    ///    is not more than `days` days later than the start of the year -- it is considered that
    ///    `self` belongs to the previous year.
    /// 3. Otherwise we consider `self` belonging to the same year today's date belongs to.
    ///
    /// # Errors
    ///
    /// * If `days` is not in `1..=31` -- [`Error::InvalidAdaptRange`] is returned.
    /// * If [`Self::ordinal()`] is 366 and the algorithm has decided that `self` belongs to
    ///   a non-leap year -- [`Error::OverflowNotLeapYear`] is returned.
    ///
    /// # Examples
    ///
    /// Let's assume it's 364'th day of 2015 and we are shown various boarding passes. The boarding passes
    /// don't contain precise dates, instead they contain day of year only (i.e. only the ordinal day).
    ///
    /// it is also a common knowledge, that people can buy tickets with boarding that happens next year.
    /// Because of that, day 4 doesn't mean it's for 2015. It might be a boarding pass for 2016! Strictly
    /// speaking, we will never learn if it's true or not, but assuming that such boarding pass is for 2016
    /// is a good educated guess.
    ///
    /// Of course, this rule shouldn't be abused and that's where the `days` arguments comes in.
    ///
    /// ```
    /// use iata::datetime::DayOfYear;
    ///
    /// let date = DayOfYear::new(364).unwrap().to_date(2015).unwrap();
    ///
    /// // It's day `4`, `days` (the allowed range) is 7. The function will
    /// // assume it's the boarding pass for the next year.
    /// assert_eq!(
    ///     DayOfYear::new(4).unwrap().to_date_adapt(date, 7),
    ///     DayOfYear::new(4).unwrap().to_date(2016),
    /// );
    /// // It's day `4`, `days` (the allowed range) is 2. The function will
    /// // assume it's the boarding pass for this year, since the date is out
    /// // of the allowed range.
    /// assert_eq!(
    ///     DayOfYear::new(4).unwrap().to_date_adapt(date, 2),
    ///     DayOfYear::new(4).unwrap().to_date(2015),
    /// );
    /// ```
    ///
    /// Of course, the situation can be complete 180. It might be the 4th day of 2015 and
    /// the boarding pass seems to contain the date far in the future. In that case it would
    /// be good to assume that this boarding pass was actually for the past year!
    ///
    /// ```
    /// use iata::datetime::DayOfYear;
    ///
    /// let date = DayOfYear::new(4).unwrap().to_date(2015).unwrap();
    ///
    /// assert_eq!(
    ///     DayOfYear::new(364).unwrap().to_date_adapt(date, 7),
    ///     DayOfYear::new(364).unwrap().to_date(2014),
    /// );
    /// ```
    pub fn to_date_adapt(&self, for_date: Date, days: u8) -> Result<Date, Error> {
        if days == 0 || days > MAX_ADAPT_DAYS {
            return Err(Error::InvalidAdaptRange(days));
        }

        let mut year: i32 = for_date.year() as i32;
        let for_ordinal = for_date.day_of_year() as u16;

        let days = days as u16;
        let upper_limit = 365 - days;
        if self.0 < days && for_ordinal > upper_limit {
            year += 1;
        } else if self.0 > upper_limit && for_ordinal < days {
            year -= 1;
        }

        if self.0 == 366 && !is_leap_year(year) {
            return Err(Error::OverflowNotLeapYear);
        }

        self.to_date(year)
    }
}

impl Default for DayOfYear {
    fn default() -> Self {
        Self(1)
    }
}
