use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let mut count: i32 = 0;
	let mut prev: i64;
	let fh = File::open("input");
	let f: std::fs::File;
	match fh {
		Ok(v) => f = v,
		Err(e) => {
			println!("{}", e);
			return ()
		},
	}
	let mut buf = BufReader::new(f);	

	let mut done = false;
	let mut ln = String::new();
	let _result = buf.read_line(&mut ln);
	ln.pop();
	let init = ln.parse();
	match init {
		Ok(some) => prev = some,
		Err(_e) => {
			println!("error with initial line: {}", ln);
			return ();
		},
	}
	ln.clear();	
	while !done {
		let rl = buf.read_line(&mut ln);
		match rl {
			Ok(ri) => {
				if ri <= 1 {
					done = true;
					continue;
				}
				ln.pop();
				let cur: i64 = ln.parse().unwrap();
				if cur > prev {
					count = count + 1;
				}
				prev = cur;
				ln.clear();
			},
			Err(e) => {
				println!("{}", e);
				done = true;
			},
		}
	}
	println!("counted: {}", count);
	();
}
