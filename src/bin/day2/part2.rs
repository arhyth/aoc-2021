use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let f: std::fs::File;
	if let Ok(some) = File::open("input") {
		f = some
	} else {
		return ()
	}
	let mut buf = BufReader::new(f);

	let (mut fwd, mut depth, mut aim) = (0, 0, 0);
	let mut ln = String::new();
	let mut count: i32;
	let mut trimmed: &str;
	const FORWARD: &str = "forward";
	const UP: &str = "up";
	const DOWN: &str = "down";

	while let Ok(ri) = buf.read_line(&mut ln) {
		if ri <= 1 {
			break;
		}
		trimmed = ln.trim();
		let cmd: Vec<&str> = trimmed.split(" ").collect();
		if let Ok(c) = cmd[1].parse() {
			count = c
		} else {
			println!("terminating: count missing/invalid");
			break;
		}
		match cmd[0] {
			FORWARD => {
				fwd += count;
                depth += aim * count;
			},
			UP => aim -= count,
			DOWN => aim += count,
			_other => {
                println!("terminating: unhandled action")
            }
		}

		ln.clear();
	}

	println!("distance covered: {}", fwd * depth);
}
