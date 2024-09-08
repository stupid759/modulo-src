// rust TeleType

use chrono::Local;

pub fn teletype(string: &str) {
	let now = Local::now();
	let timestamp = now.format("%H:%M:%S").to_string();
	println!("{timestamp} | {string}");
}
