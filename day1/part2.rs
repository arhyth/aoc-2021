use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let mut count: i32 = 0;
	let f: std::fs::File;
	if let Ok(some) = File::open("input") {
		f = some
	} else {
		return ()
	}
	let mut buf = BufReader::new(f);	

	let (mut c1, mut c2, mut c3, mut cur, mut prev): (i32, i32, i32, i32, i32);
	let mut ln = String::new();

	let _result = buf.read_line(&mut ln);
	ln.pop();
	let init = ln.parse();
	if let Ok(some) = init {
		c1 = some;
	} else {
		return ();
	}
	ln.clear();
	
	let _result = buf.read_line(&mut ln);
	ln.pop();
	let init = ln.parse();
	if let Ok(some) = init {
		c2 = some;
	} else {
		return ();
	}
	ln.clear();
	let _result = buf.read_line(&mut ln);
	ln.pop();
	let init = ln.parse();
	if let Ok(some) = init {
		c3 = some;
	} else {
		return ();
	}
	ln.clear();
	prev = c1 + c2 + c3;

	while let Ok(ri) = buf.read_line(&mut ln) {
		if ri <= 1 {
			break;
		}
		ln.pop();
		c1 = c2;
		c2 = c3;
		c3 = ln.parse().unwrap();
		cur = c1 + c2 + c3;
		if cur > prev {
			count = count + 1;
		}
		prev = cur;
		ln.clear();
	}
	
	println!("counted: {}", count);
}
