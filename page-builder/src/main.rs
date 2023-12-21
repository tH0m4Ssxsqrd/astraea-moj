
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
	let mut file = File::open("input.html").expect("Could not open file");
	let mut file_content = String::new();
	file.read_to_string(&mut file_content)
		.expect("Could not read from file");

	let start_time = Instant::now();

	let name = "John Doe";
	let class = "CS101";
	let task = "Task 1";

	let mut new_content = file_content;
	while new_content.contains("$NAME") || new_content.contains("$CLASS") || new_content.contains("$TASK") {
		new_content = new_content.replace("$NAME", name);
		new_content = new_content.replace("$CLASS", class);
		new_content = new_content.replace("$TASK", task);
	}

	let end_time = Instant::now();
	let elapsed_time = end_time.duration_since(start_time);

	println!("Execution time: {elapsed_time:?}");

	let mut file = File::create("input.html").expect("Could not create file");
	file.write_all(new_content.as_bytes()).expect("Could not write file");
}
