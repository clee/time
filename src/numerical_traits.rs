use crate::Duration;
use core::time::Duration as StdDuration;

/// Create `Duration`s from primitive and core numeric types.
///
/// This trait can be imported (alongside others) with `use time::prelude::*`.
///
/// Due to limitations in rustc, these methods are currently _not_ `const fn`.
///
/// # Examples
///
/// Basic construction of `Duration`s.
///
/// ```rust
/// # use time::{Duration, NumericalDuration};
/// assert_eq!(5.nanoseconds(), Duration::nanoseconds(5));
/// assert_eq!(5.microseconds(), Duration::microseconds(5));
/// assert_eq!(5.milliseconds(), Duration::milliseconds(5));
/// assert_eq!(5.seconds(), Duration::seconds(5));
/// assert_eq!(5.minutes(), Duration::minutes(5));
/// assert_eq!(5.hours(), Duration::hours(5));
/// assert_eq!(5.days(), Duration::days(5));
/// assert_eq!(5.weeks(), Duration::weeks(5));
/// ```
///
/// Signed integers work as well!
/// ```rust
/// # use time::{Duration, NumericalDuration};
/// assert_eq!(-5.nanoseconds(), Duration::nanoseconds(-5));
/// assert_eq!(-5.microseconds(), Duration::microseconds(-5));
/// assert_eq!(-5.milliseconds(), Duration::milliseconds(-5));
/// assert_eq!(-5.seconds(), Duration::seconds(-5));
/// assert_eq!(-5.minutes(), Duration::minutes(-5));
/// assert_eq!(-5.hours(), Duration::hours(-5));
/// assert_eq!(-5.days(), Duration::days(-5));
/// assert_eq!(-5.weeks(), Duration::weeks(-5));
/// ```
///
/// Just like any other `Duration`, they can be added, subtracted, etc.
///
/// ```rust
/// # use time::NumericalDuration;
/// assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
/// assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
/// ```
///
/// As floats can only be used to construct via anything other than
/// `Duration::seconds_f32` and `Duration::seconds_f64`, floats _do not_
/// implement `NumericalDuration`.
///
/// ```rust,compile_fail
/// # use time::NumericalDuration;
/// 5.0.seconds();
/// ```
pub trait NumericalDuration {
    /// Create a `Duration` from the number of nanoseconds.
    fn nanoseconds(self) -> Duration;
    /// Create a `Duration` from the number of microseconds.
    fn microseconds(self) -> Duration;
    /// Create a `Duration` from the number of milliseconds.
    fn milliseconds(self) -> Duration;
    /// Create a `Duration` from the number of seconds.
    fn seconds(self) -> Duration;
    /// Create a `Duration` from the number of minutes.
    fn minutes(self) -> Duration;
    /// Create a `Duration` from the number of hours.
    fn hours(self) -> Duration;
    /// Create a `Duration` from the number of days.
    fn days(self) -> Duration;
    /// Create a `Duration` from the number of weeks.
    fn weeks(self) -> Duration;
}

macro_rules! impl_numerical_duration {
    ($($type:ty),* $(,)?) => {
        $(
            #[allow(trivial_numeric_casts, clippy::use_self)]
            impl NumericalDuration for $type {
                fn nanoseconds(self) -> Duration {
                    Duration::nanoseconds(self as i64)
                }

                fn microseconds(self) -> Duration {
                    Duration::microseconds(self as i64)
                }

                fn milliseconds(self) -> Duration {
                    Duration::milliseconds(self as i64)
                }

                fn seconds(self) -> Duration {
                    Duration::seconds(self as i64)
                }

                fn minutes(self) -> Duration {
                    Duration::minutes(self as i64)
                }

                fn hours(self) -> Duration {
                    Duration::hours(self as i64)
                }

                fn days(self) -> Duration {
                    Duration::days(self as i64)
                }

                fn weeks(self) -> Duration {
                    Duration::weeks(self as i64)
                }
            }
        )*
    };
}

macro_rules! impl_numerical_duration_nonzero {
    ($($type:ty),* $(,)?) => {
        $(
            #[allow(trivial_numeric_casts, clippy::use_self)]
            impl NumericalDuration for $type {
                fn nanoseconds(self) -> Duration {
                    Duration::nanoseconds(self.get() as i64)
                }

                fn microseconds(self) -> Duration {
                    Duration::microseconds(self.get() as i64)
                }

                fn milliseconds(self) -> Duration {
                    Duration::milliseconds(self.get() as i64)
                }

                fn seconds(self) -> Duration {
                    Duration::seconds(self.get() as i64)
                }

                fn minutes(self) -> Duration {
                    Duration::minutes(self.get() as i64)
                }

                fn hours(self) -> Duration {
                    Duration::hours(self.get() as i64)
                }

                fn days(self) -> Duration {
                    Duration::days(self.get() as i64)
                }

                fn weeks(self) -> Duration {
                    Duration::weeks(self.get() as i64)
                }
            }
        )*
    };
}

impl_numerical_duration![u8, u16, u32, i8, i16, i32, i64];
impl_numerical_duration_nonzero![
    core::num::NonZeroU8,
    core::num::NonZeroU16,
    core::num::NonZeroU32,
    core::num::NonZeroI8,
    core::num::NonZeroI16,
    core::num::NonZeroI32,
    core::num::NonZeroI64,
];

/// Create `std::time::Duration`s from primitive and core numeric types.
///
/// This trait can be imported (alongside others) with `use time::prelude::*`.
///
/// Due to limitations in rustc, these methods are currently _not_ `const fn`.
///
/// # Examples
///
/// Basic construction of `std::time::Duration`s.
///
/// ```rust
/// # use time::NumericalStdDuration;
/// # use core::time::Duration;
/// assert_eq!(5u32.std_nanoseconds(), Duration::from_nanos(5));
/// assert_eq!(5u32.std_microseconds(), Duration::from_micros(5));
/// assert_eq!(5u32.std_milliseconds(), Duration::from_millis(5));
/// assert_eq!(5u32.std_seconds(), Duration::from_secs(5));
/// assert_eq!(5u32.std_minutes(), Duration::from_secs(5 * 60));
/// assert_eq!(5u32.std_hours(), Duration::from_secs(5 * 3_600));
/// assert_eq!(5u32.std_days(), Duration::from_secs(5 * 86_400));
/// assert_eq!(5u32.std_weeks(), Duration::from_secs(5 * 604_800));
/// ```
///
/// Just like any other `std::time::Duration`, they can be added, subtracted,
/// etc.
///
/// ```rust
/// # use time::NumericalStdDuration;
/// assert_eq!(2u32.std_seconds() + 500u32.std_milliseconds(), 2_500u32.std_milliseconds());
/// assert_eq!(2u32.std_seconds() - 500u32.std_milliseconds(), 1_500u32.std_milliseconds());
/// ```
///
/// As floats can only be used to construct via anything other than
/// `Duration::from_secs_f32` and `Duration::from_secs_f64`, floats _do not_
/// implement `NumericalStdDuration`.
///
/// ```rust,compile_fail
/// # use time::NumericalStdDuration;
/// 5.0.std_seconds();
/// ```
pub trait NumericalStdDuration {
    /// Create a `std::time::Duration` from the number of nanoseconds.
    fn std_nanoseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of microseconds.
    fn std_microseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of milliseconds.
    fn std_milliseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of seconds.
    fn std_seconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of minutes.
    fn std_minutes(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of hours.
    fn std_hours(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of days.
    fn std_days(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of weeks.
    fn std_weeks(self) -> StdDuration;
}

macro_rules! impl_numerical_std_duration {
    ($($type:ty),* $(,)?) => {
        $(
            #[allow(trivial_numeric_casts, clippy::use_self)]
            impl NumericalStdDuration for $type {
                fn std_nanoseconds(self) -> StdDuration {
                    StdDuration::from_nanos(self as u64)
                }

                fn std_microseconds(self) -> StdDuration {
                    StdDuration::from_micros(self as u64)
                }

                fn std_milliseconds(self) -> StdDuration {
                    StdDuration::from_millis(self as u64)
                }

                fn std_seconds(self) -> StdDuration {
                    StdDuration::from_secs(self as u64)
                }

                fn std_minutes(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 60)
                }

                fn std_hours(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 3_600)
                }

                fn std_days(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 86_400)
                }

                fn std_weeks(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 604_800)
                }
            }
        )*
    };
}

macro_rules! impl_numerical_std_duration_nonzero {
    ($($type:ty),* $(,)?) => {
        $(
            #[allow(trivial_numeric_casts, clippy::use_self)]
            impl NumericalStdDuration for $type {
                fn std_nanoseconds(self) -> StdDuration {
                    StdDuration::from_nanos(self.get() as u64)
                }

                fn std_microseconds(self) -> StdDuration {
                    StdDuration::from_micros(self.get() as u64)
                }

                fn std_milliseconds(self) -> StdDuration {
                    StdDuration::from_millis(self.get() as u64)
                }

                fn std_seconds(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64)
                }

                fn std_minutes(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 60)
                }

                fn std_hours(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 3_600)
                }

                fn std_days(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 86_400)
                }

                fn std_weeks(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 604_800)
                }
            }
        )*
    };
}

impl_numerical_std_duration![u8, u16, u32, u64];
impl_numerical_std_duration_nonzero![
    core::num::NonZeroU8,
    core::num::NonZeroU16,
    core::num::NonZeroU32,
    core::num::NonZeroU64,
];
