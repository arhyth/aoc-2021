extern crate clap;

use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::{BufRead, BufReader};

fn main() {
	let args = App::new("Day 3 solution for Advent of Code")
		.version("0.1.0")
		.author("David Ebreo <arhyth@gmail.com>")
		.subcommand(SubCommand::with_name("one")
			.about("part 1"))
		.subcommand(SubCommand::with_name("two")
			.about("part 2"))
		.arg(Arg::with_name("file")
			.short("f")
			.long("file")
			.value_name("FILE")
			.help("Sets input file")
			.default_value("input")
			.takes_value(true))
		.get_matches();

	let f: std::fs::File;
	if let Some(fname) = args.value_of("file") {
		if let Ok(some) = File::open(fname) {
			f = some
		} else {
			return ()
		}
	} else {
		return ()
	}

	let buf: &mut dyn BufRead;
	let mut bo: Box<BufReader<std::fs::File>>;
	let mut bs: Basa;
	let mut inner: Vec<u8> = vec![97; 10];

	match args.subcommand() {
		("one", _) => {
			bo = Box::new(BufReader::new(f));
			buf = &mut bo;
		},
		("two", _) => {
			bs = Basa{count: 0, buf: &mut inner[0..9]};
			buf = &mut bs;
		},
		_any => {
			println!("f* this");
			bs = Basa{count: 0, buf: &mut inner[0..9]};
			buf = &mut bs;
		},
	}

	one(buf);
}

#[derive(Clone)]
struct ZerOne(i32, i32);

struct Basa<'a> {
    count: usize,
    buf: &'a mut [u8],
}

impl<'a> Read for Basa<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
	println!("log read");
	if self.count <= 10 {
	    let mut ln = 0;
	    println!("buffered: {}", buf.len());
	    while ln < buf.len() {
	        println!("idx: {}", ln);
	        if let Some(it) = buf.get_mut(ln) {
		    	*it = 97;
	        }
	        ln += 1
	    }
		self.count += 1;
		Ok(buf.len())
	    } else {
			println!("done?");
			Ok(0)
	    }
    }
}

impl<'a> BufRead for Basa<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
	println!("fill buffer: {}", self.count);
	if self.count > 10 {
	    self.buf = &mut [];
	} else {
	    self.count += 1
	}
        Ok(&self.buf)
    }

    fn consume(&mut self, amt: usize) {
		println!("consume: {}", amt);
        ()
    }
}

fn one(buf: &mut dyn BufRead) -> () {
	let mut ln = String::with_capacity(12);
	let mut counters: Vec<ZerOne> = vec![ZerOne(0, 0); 12];

	while let Ok(ri) = buf.read_line(&mut ln) {
		if ri <= 1 {
			break;
		}
		let bits = ln.as_bytes();
		let mut idx: usize = 0;
		while idx < 12 {
			match bits[idx] {
				48 => {
					if let Some(c) = counters.get_mut(idx) {
						c.0 = c.0 + 1;
					}
				},
				49 => {
					if let Some(c) = counters.get_mut(idx) {
						c.1 = c.1 + 1;
					}
				},
				_any => (),
			}

			idx += 1;
		}
		ln.clear();
	}

	// let (mut gamma, mut epsilon) = (0b0, 0b0);
	let mut gamma: u32 = 0b0;
	let epsilon: u32;
	for c in counters.iter() {
		gamma <<= 1;
		// println!("counters everywhere!\n({}, {})", c.0, c.1);
		if c.1 > c.0 {
			gamma += 0b1;
		}
	}

	// invert and turnoff unused bits
	epsilon = !gamma - 0b11111111111111111111000000000000;
	println!("({}, {})", gamma, epsilon);
}

fn two<'a>(mut _buf: impl BufRead + 'a) -> () {
	todo!()
}
