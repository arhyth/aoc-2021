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

	let (mut fwd, mut depth) = (0, 0);
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
			println!("weee woooh");
			break;
		}
		match cmd[0] {
			FORWARD => {
				fwd += count;
			},
			UP => {
				depth -= count;
			},
			DOWN => depth += count,
			_other => println!("seriously???")
		}

		ln.clear();
	}
	
	println!("forward, depth: {}, {}", fwd, depth);
}
