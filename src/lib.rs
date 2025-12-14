use libc::c_int;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct TimeBase {
    numer: u32,
    denom: u32,
}

unsafe extern "C" {
    /// https://developer.apple.com/documentation/driverkit/mach_timebase_info-c.func
    fn mach_timebase_info(info: *mut TimeBase) -> c_int;

    /// https://developer.apple.com/documentation/driverkit/mach_absolute_time
    fn mach_absolute_time() -> u64;
}

static mut TIME_BASE: Option<TimeBase> = None;

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

fn get_monotonic_nanos() -> u128 {
    let TimeBase { numer, denom } = get_timebase();
    let ticks = unsafe { mach_absolute_time() };

    // Convert to nanosecond.
    let numer = numer as u128;
    let denom = denom as u128;
    (ticks as u128 * numer) / denom
}

/// Get high resolution wall's nanosecond timestamp, based on system time and monotonic nanos.
struct HighResClock {
    base_time_ns: u128,
    base_monotonic_ns: u128,
}

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

static HIGH_RES_CLOCK: OnceLock<HighResClock> = OnceLock::new();

pub fn get_timestamp_ns() -> u128 {
    let clock = HIGH_RES_CLOCK.get_or_init(HighResClock::new);
    clock.now_nanos()
}
