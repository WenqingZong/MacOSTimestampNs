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

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("This library only works on macos, it does nothing on other platforms");
}
