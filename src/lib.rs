//! MacOS only provides timestamps in microsecond precision, which might not be precious enough in
//! some situations. This crate provides the current time timestamp in nanosecond precision.

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use libc::c_int;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct TimeBase {
    numer: u32,
    denom: u32,
}

#[cfg(any(target_os = "macos", docsrs))]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
unsafe extern "C" {
    /// https://developer.apple.com/documentation/driverkit/mach_timebase_info-c.func
    fn mach_timebase_info(info: *mut TimeBase) -> c_int;

    /// https://developer.apple.com/documentation/driverkit/mach_absolute_time
    fn mach_absolute_time() -> u64;
}

static mut TIME_BASE: Option<TimeBase> = None;

#[cfg(any(target_os = "macos", docsrs))]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
fn get_timebase() -> TimeBase {
    unsafe {
        TIME_BASE.unwrap_or_else(|| {
            let mut timebase = TimeBase { numer: 0, denom: 0 };
            mach_timebase_info(&mut timebase);
            TIME_BASE = Some(timebase);
            timebase
        })
    }
}

#[cfg(any(target_os = "macos", docsrs))]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
fn get_monotonic_nanos() -> u128 {
    let TimeBase { numer, denom } = get_timebase();
    let ticks = unsafe { mach_absolute_time() };

    // Convert to nanosecond.
    let numer = numer as u128;
    let denom = denom as u128;
    (ticks as u128 * numer) / denom
}

#[cfg(any(target_os = "macos", docsrs))]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
/// Get high resolution wall's nanosecond timestamp, based on system time and monotonic nanos.
struct HighResClock {
    base_time_ns: u128,
    base_monotonic_ns: u128,
}

#[cfg(any(target_os = "macos", docsrs))]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
impl HighResClock {
    fn new() -> Self {
        let base_time_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let base_monotonic_ns = get_monotonic_nanos();

        HighResClock {
            base_time_ns,
            base_monotonic_ns,
        }
    }

    fn now_nanos(&self) -> u128 {
        let current_monotonic = get_monotonic_nanos();
        let elapsed = current_monotonic - self.base_monotonic_ns;

        self.base_time_ns + elapsed
    }
}

#[cfg(any(target_os = "macos", docsrs))]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
static HIGH_RES_CLOCK: OnceLock<HighResClock> = OnceLock::new();

/// Get the current nanosecond timestamp as an [`u128`].
/// # Example
/// ```rust
/// use macos_timestamp_ns::get_timestamp_ns;
///
/// # fn main() {
/// let ts = get_timestamp_ns();
/// assert_eq!(ts.to_string().len(), 19);
/// # }
/// ```
#[cfg(any(target_os = "macos", docsrs))]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
pub fn get_timestamp_ns() -> u128 {
    let clock = HIGH_RES_CLOCK.get_or_init(HighResClock::new);
    clock.now_nanos()
}

/// Get the current nanosecond timestamp as [`DateTime<Utc>`].
/// # Example
/// ```rust
/// #[cfg(feature = "chrono")]
/// use macos_timestamp_ns::get_timestamp_ns_datetime;
///
/// # fn main() {
/// #[cfg(feature = "chrono")]
/// {
///     let ts = get_timestamp_ns_datetime();
///     assert_eq!(ts.timestamp_nanos_opt().unwrap().to_string().len(), 19);
/// }
/// # }
/// ```
#[cfg(feature = "chrono")]
#[cfg(any(target_os = "macos", docsrs))]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
pub fn get_timestamp_ns_datetime() -> DateTime<Utc> {
    let ts_ns = get_timestamp_ns();
    DateTime::from_timestamp_nanos(ts_ns as i64)
}
