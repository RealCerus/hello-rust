use std::time;

fn main() {
    print_time_in_millis();
}

fn print_time_in_millis() {
    let now = time::SystemTime::now();
    let now_in_ms = now.duration_since(time::UNIX_EPOCH).expect("WTF").as_millis();
    println!("{}", now_in_ms);
}