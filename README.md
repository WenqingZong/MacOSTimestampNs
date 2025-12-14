# MacOSTimestampNs
Provide nano second timestamps for MacOS, as `SystemTime::now()` only has microsecond precision. 

## Kind Reminder
This library is only intended to be used in `MacOS`. It does nothing in other platforms. 

## Example
```rust
use macos_timestamp_ns::get_timestamp_ns;
#[cfg(feature = "chrono")]
use macos_timestamp_ns::get_timestamp_ns_datetime;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(target_os = "macos")]
fn main() {
    let mut timestamps_ns = vec![];
    for _ in 0..10 {
        timestamps_ns.push((
            "SystemTime\t\t",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        sleep(Duration::from_nanos(100));
        timestamps_ns.push(("This Crate\t\t", get_timestamp_ns()));
        #[cfg(feature = "chrono")]
        timestamps_ns.push((
            "This Crate (DateTime)\t",
            get_timestamp_ns_datetime().timestamp_nanos_opt().unwrap() as u128,
        ));
    }

    for (i, (source, ts)) in timestamps_ns.iter().enumerate() {
        println!("{} - {}", source, ts);
        #[cfg(not(feature = "chrono"))]
        if i % 2 == 1 {
            println!();
        }
        #[cfg(feature = "chrono")]
        if i % 3 == 2 {
            println!()
        }
    }
}
```

This will give you: 
```text
SystemTime               - 1765729262738534000
This Crate               - 1765729262738566083
This Crate (DateTime)    - 1765729262738566250

SystemTime               - 1765729262738567000
This Crate               - 1765729262738574500
This Crate (DateTime)    - 1765729262738575375

SystemTime               - 1765729262738576000
This Crate               - 1765729262738582625
This Crate (DateTime)    - 1765729262738582667

SystemTime               - 1765729262738583000
This Crate               - 1765729262738590000
This Crate (DateTime)    - 1765729262738590042

SystemTime               - 1765729262738590000
This Crate               - 1765729262738596792
This Crate (DateTime)    - 1765729262738596833

SystemTime               - 1765729262738597000
This Crate               - 1765729262738603625
This Crate (DateTime)    - 1765729262738604042

SystemTime               - 1765729262738604000
This Crate               - 1765729262738610958
This Crate (DateTime)    - 1765729262738611042

SystemTime               - 1765729262738611000
This Crate               - 1765729262738617792
This Crate (DateTime)    - 1765729262738617833

SystemTime               - 1765729262738618000
This Crate               - 1765729262738624625
This Crate (DateTime)    - 1765729262738624667

SystemTime               - 1765729262738625000
This Crate               - 1765729262738631417
This Crate (DateTime)    - 1765729262738631458
```

## Contributions
Issues, bug reports, suggestions, PRs, all welcomed!
