use jiff::{
    civil::{Date, DateTime},
    tz::Offset,
};

use super::{Error, Month, MonthDay, Time, TimeZoneIndicator};

/// A calendar month and day paired with a time of day.
#[derive(Clone, Debug, PartialEq)]
pub struct MonthDayTime {
    pub date: MonthDay,
    pub time: Time,
}

impl MonthDayTime {
    /// Constructs a new instance of [`MonthDayTime`]. This function is equivalent
    /// to simply constructing the value.
    ///
    /// ```
    /// use iata::datetime::{Month, MonthDay, MonthDayTime, Time, TimeZoneIndicator};
    ///
    /// assert_eq!(
    ///     MonthDayTime::new(
    ///         MonthDay::new(Month::September, 1).unwrap(),
    ///         Time::new(20, 45, Some(12), TimeZoneIndicator::None).unwrap(),
    ///     ),
    ///     MonthDayTime {
    ///         date: MonthDay::new(Month::September, 1).unwrap(),
    ///         time: Time::new(20, 45, Some(12), TimeZoneIndicator::None).unwrap(),
    ///     },
    /// );
    /// ```
    pub fn new(date: MonthDay, time: Time) -> Self {
        Self { date, time }
    }

    /// Constructs a new instance of [`MonthDayTime`], with an optional time. The
    /// [`None`] value is mapped into [`Time::default()`].
    pub fn new_time_opt(date: MonthDay, time: Option<Time>) -> Self {
        let time = time.unwrap_or_default();

        Self::new(date, time)
    }

    /// Returns an immutable reference to the date component.
    pub fn date(&self) -> &MonthDay {
        &self.date
    }

    /// A shortcut method that returns the month component of the date.
    pub fn month(&self) -> Month {
        self.date.month()
    }

    /// A shortcut method that returns the day component of the date.
    pub fn day(&self) -> u8 {
        self.date.day()
    }

    /// A shortcut method that returns the hour component of the time.
    pub fn hour(&self) -> u8 {
        self.time.hour()
    }

    /// A shortcut method that returns the minute component of the time.
    pub fn minute(&self) -> u8 {
        self.time.minute()
    }

    /// A shortcut method that returns the second component of the time.
    pub fn second(&self) -> Option<u8> {
        self.time.second()
    }

    /// A shortcut method that returns the timezone component of the time.
    pub fn timezone(&self) -> TimeZoneIndicator {
        self.time.timezone()
    }

    /// Converts the struct into [`jiff::civil::DateTime`], using the supplied
    /// year.
    ///
    /// # Errors
    ///
    /// This method returns an error only if `self.date.to_date(year)` fails. For more
    /// information see [`MonthDay::to_date()`].
    pub fn to_datetime(&self, year: i16) -> Result<DateTime, Error> {
        self.date
            .to_date(year)
            .map(|date| date.to_datetime(self.time.to_time()))
    }

    /// Tries to figure out what year the date-time belongs to. Equivalent to
    /// `self.to_datetime_adapt(today, days)`.
    ///
    /// For more information about algorithm and some examples, see
    /// [`MonthDay::to_date_adapt`].
    pub fn to_datetime_adapt_year(&self, offset: Offset, days: u8) -> Result<DateTime, Error> {
        self.date
            .to_date_adapt_year(offset, days)
            .map(|date| date.to_datetime(self.time.to_time()))
    }

    /// Tries to figure out what year the date-time belongs to.
    ///
    /// For more information about algorithm and some examples, see
    /// [`MonthDay::to_date_adapt`].
    pub fn to_datetime_adapt(&self, for_date: Date, days: u8) -> Result<DateTime, Error> {
        self.date
            .to_date_adapt(for_date, days)
            .map(|date| date.to_datetime(self.time.to_time()))
    }
}
