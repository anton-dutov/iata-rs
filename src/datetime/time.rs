use std::str::FromStr;

use jiff::civil::Time as JiffTime;

use super::{Error, TimeZoneIndicator};

/// A struct that stores HH:MM together with its timezone, optionally
/// including the seconds.
#[derive(Clone, Debug, PartialEq)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: Option<u8>,
    timezone: TimeZoneIndicator,
}

impl Time {
    /// Constructs the struct out of given values.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::InvalidHourValue`] if `hour` isn't in the `0..24` range.
    /// * Returns [`Error::InvalidMinuteValue`] if `minute` isn't in the `0..=59` range.
    /// * Returns [`Error::InvalidSecondValue`] if `second` is [`Some`] and the value isn't
    ///   in the `0..=59` range.
    pub fn new(
        hour: u8,
        minute: u8,
        second: Option<u8>,
        timezone: TimeZoneIndicator,
    ) -> Result<Self, Error> {
        if hour > 23 {
            return Err(Error::InvalidHourValue(hour));
        }

        if minute > 59 {
            return Err(Error::InvalidMinuteValue(minute));
        }

        if let Some(second) = second {
            if second > 59 {
                return Err(Error::InvalidSecondValue(second));
            }
        }

        Ok(Self {
            hour,
            minute,
            second,
            timezone,
        })
    }

    /// Returns the hour component.
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Returns the minute component.
    pub fn minute(&self) -> u8 {
        self.minute
    }

    /// Returns the second component.
    pub fn second(&self) -> Option<u8> {
        self.second
    }

    /// Returns the timezone.
    pub fn timezone(&self) -> TimeZoneIndicator {
        self.timezone
    }

    /// Converts this struct into [`jiff::civil::Time`].
    pub fn to_time(&self) -> JiffTime {
        JiffTime::new(
            self.hour as i8,
            self.minute as i8,
            self.second.unwrap_or_default() as i8,
            0,
        )
        .unwrap()
    }

    /// Parses a time in `HHMM` format.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::InvalidInput`] if the input has a length other than 4 bytes
    ///   or a UTF-8 character crosses a field boundary.
    /// * If the first two bytes of the string can't be parsed as a non-negative decimal
    ///   integer -- [`Error::InvalidHour`] is returned.
    /// * If the second two bytes of the string can't be parsed as a non-negative decimal
    ///   integer -- [`Error::InvalidMinute`] is returned.
    /// * If the parsed values don't denote a valid time -- same errors as in [`Self::new()`] are
    ///   returned.
    pub fn from_short_str(s: &str) -> Result<Self, Error> {
        if s.len() != 4 {
            return Err(Error::InvalidInput(s.to_owned()));
        }

        let (hour, minute) = s
            .split_at_checked(2)
            .ok_or_else(|| Error::InvalidInput(s.to_owned()))?;

        let hour = hour
            .parse()
            .map_err(|_| Error::InvalidHour(hour.to_owned()))?;

        let minute = minute
            .parse()
            .map_err(|_| Error::InvalidMinute(minute.to_owned()))?;

        Self::new(hour, minute, None, TimeZoneIndicator::None)
    }

    /// Parses a time in `HHMMSST` or `HHMMT` format, where `T` is the zone suffix.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::InvalidInput`] if the input has a length other than 5 or 7 bytes
    ///   or a UTF-8 character crosses a field boundary.
    /// * If the first two bytes of the string can't be parsed as a non-negative decimal
    ///   integer -- [`Error::InvalidHour`] is returned.
    /// * If the second two bytes of the string can't be parsed as a non-negative decimal
    ///   integer -- [`Error::InvalidMinute`] is returned.
    /// * If the seconds part is present and can't be parsed as a non-negative decimal
    ///   integer -- [`Error::InvalidSecond`] is returned.
    /// * If timezone suffix is not recognized -- [`Error::InvalidTimezoneTag`] is returned.
    /// * If the parsed values don't denote a valid time -- same errors as in [`Self::new()`] are
    ///   returned.
    pub fn from_full_str(s: &str) -> Result<Self, Error> {
        let invalid_input = || Error::InvalidInput(s.to_owned());
        if s.len() != 5 && s.len() != 7 {
            return Err(invalid_input());
        }

        let (hour, tail) = s.split_at_checked(2).ok_or_else(invalid_input)?;
        let (minute, tail) = tail.split_at_checked(2).ok_or_else(invalid_input)?;

        let hour = hour
            .parse()
            .map_err(|_| Error::InvalidHour(hour.to_owned()))?;

        let minute = minute
            .parse()
            .map_err(|_| Error::InvalidMinute(minute.to_owned()))?;

        if tail.len() > 1 {
            // We have seconds and timezone
            let (seconds, tz) = tail.split_at_checked(2).ok_or_else(invalid_input)?;

            let seconds = seconds
                .parse::<u8>()
                .map_err(|_| Error::InvalidSecond(seconds.to_owned()))?;

            Self::new(
                hour,
                minute,
                Some(seconds),
                TimeZoneIndicator::from_str(tz)?,
            )
        } else {
            // We have timezone only
            let tz = tail;

            Self::new(hour, minute, None, TimeZoneIndicator::from_str(tz)?)
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: None,
            timezone: TimeZoneIndicator::None,
        }
    }
}
