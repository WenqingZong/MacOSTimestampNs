use macos_timestamp_ns::get_timestamp_ns;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let mut timestamps_ns = vec![];
    for _ in 0..10 {
        timestamps_ns.push((
            "SystemTime",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        sleep(Duration::from_nanos(100));
        timestamps_ns.push(("This Crate", get_timestamp_ns()));
    }

    for (i, (source, ts)) in timestamps_ns.iter().enumerate() {
        println!("{} - {}", source, ts);
        if i % 2 == 1 {
            println!();
        }
    }
}
