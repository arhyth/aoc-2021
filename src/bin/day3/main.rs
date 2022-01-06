extern crate clap;

use clap::{App, Arg, SubCommand};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;

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

	let buf = &mut BufReader::new(f);
	let part_fn: fn(&mut BufReader<std::fs::File>) -> Result<(), bool>;

	match args.subcommand() {
		("one", _) => {
			part_fn = one;
		},
		("two", _) => {
			part_fn = two;
		},
		_any => {
			return ()
		},
	}

	match part_fn(buf) {
		Ok(()) => (),
		Err(_bool) => {
			println!("error encountered");
			()
		}
	}
}

#[derive(Clone)]
struct ZerOne(u16, u16);

impl fmt::Display for ZerOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0 > self.1 {
			write!(f, "0")
		} else if self.0 < self.1 {
			write!(f, "1")
		} else {
			write!(f, "b")
		}
    }
}

fn one(buf: &mut BufReader<std::fs::File>) -> Result<(), bool> {
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
				_any => return Err(false),
			}

			idx += 1;
		}
		ln.clear();
	}

	let mut gamma: u32 = 0b0;
	let epsilon: u32;
	for c in counters.iter() {
		gamma <<= 1;
		if c.1 > c.0 {
			gamma += 0b1;
		}
	}

	// invert and turnoff unused bits
	epsilon = !gamma - 0b11111111111111111111000000000000;
	println!("({}, {})", gamma, epsilon);
	Ok(())
}

struct Node {
	bit: usize,
	leaves: (u16, u16),
	val: Option<String>,
	z: Option<Box<Node>>,
	o: Option<Box<Node>>,
}

impl Node {
	fn new(bit: usize) -> Self {
		Node{
			bit,
			leaves: (0, 0),
			val: None,
			z: None,
			o: None,
		}
	}	

	fn push(&mut self, val: String) -> Result<(), bool> {
		use std::borrow::BorrowMut;

		let startbit = val.as_bytes()[0];
		match startbit {
			48 => {
				self.leaves.0 += 1;
				if self.z.is_some() {
					if let Some(borrowed_child) = self.z.as_mut() {
						let node: &mut Node = borrowed_child.borrow_mut();
						node.insert(val)?;
					}
					Ok(())
				} else {
					let mut z = Node::new(0);
					z.insert(val)?;
					self.z = Some(Box::new(z));
					Ok(())
				}
			},
			49 => {
				self.leaves.1 += 1;
				if self.o.is_some() {
					if let Some(borrowed_child) = self.o.as_mut() {
						let node: &mut Node = borrowed_child.borrow_mut();
						node.insert(val)?;
					}
					Ok(())
				} else {
					let mut o = Node::new(0);
					o.insert(val)?;
					self.o = Some(Box::new(o));
					Ok(())
				}
			},
			_any => Err(false)
		}
	}

	fn insert(&mut self, val: String) -> Result<(), bool> {
		use std::borrow::BorrowMut;

		let bits = self.bit + 1;

		if self.val.is_none() && self.o.is_none() && self.z.is_none() {
			self.val = Some(val);
			return Ok(())
		}

		match val.as_bytes()[bits] {
			48 => {
				self.leaves.0 += 1;
				if let Some(borrowed_child) = self.z.as_mut() {
					let node: &mut Node = borrowed_child.borrow_mut();
					node.insert(val)?;
				} else if self.val.is_some() {
					let cur = self.val.take().unwrap();
					if cur.as_bytes()[bits] == 48 {
						self.leaves.0 += 1;
						let mut zero = Node::new(bits);
						zero.insert(cur)?;

						self.z = Some(Box::new(zero));
					} else {
						self.leaves.1 += 1;
						let mut one = Node::new(bits);
						one.insert(cur)?;

						self.o = Some(Box::new(one));
					}

					if self.z.is_some() {
						let mut boxed = self.z.take().unwrap();
						let unboxed = boxed.as_mut();
						unboxed.insert(val)?;
						self.z = Some(boxed);
					} else {
						let mut zero = Node::new(bits);
						zero.insert(val)?;
						self.z = Some(Box::new(zero));
					}
				} else {
					let mut zero = Node::new(bits);
					zero.insert(val)?;
					self.z = Some(Box::new(zero));
				}

				Ok(())
			},
			49 => {
				self.leaves.1 += 1;
				if let Some(borrowed_child) = self.o.as_mut() {
					let node: &mut Node = borrowed_child.borrow_mut();
					node.insert(val)?;
				} else if self.val.is_some() {
					let cur = self.val.take().unwrap();
					if cur.as_bytes()[bits] == 48 {
						self.leaves.0 += 1;
						let mut zero = Node::new(bits);
						zero.insert(cur)?;

						self.z = Some(Box::new(zero));
					} else {
						self.leaves.1 += 1;
						let mut one = Node::new(bits);
						one.insert(cur)?;

						self.o = Some(Box::new(one));
					}

					if self.o.is_some() {
						let mut boxed = self.o.take().unwrap();
						let unboxed = boxed.as_mut();
						unboxed.insert(val)?;
						self.o = Some(boxed);
					} else {
						let mut one = Node::new(bits);
						one.insert(val)?;
						self.o = Some(Box::new(one));
					}
				} else {
					let mut one = Node::new(bits);
					one.insert(val)?;
					self.o = Some(Box::new(one));
				}

				Ok(())
			},
			_any => Err(false)
		}
	}

	fn find_oxygen(&self) -> Option<String> {
		let mut s = self;
		let mut repeat: u8 = 0;

		loop {
			if s.val.is_some() {
				break;
			}
			if repeat > 1 {
				break;
			}

			if s.bit == 3 {
				repeat += 1;
			}

			if s.leaves.0 > s.leaves.1 {
				if let Some(node) = &s.z {
					s = node;
				}
			} else {
				if let Some(node) = &s.o {
					s = node;
				}
			}
		}

		match s.val.clone() {
			Some(val) => {
				Some(val.clone())
			},
			None => None,
		}
	}

	fn find_carbon(&self) -> Option<String> {
		let mut s = self;

		loop {
			if s.val.is_some() {
				break;
			}

			if s.leaves.0 > s.leaves.1 {
				if let Some(node) = &s.o {
					s = node;
				} else if let Some(node) = &s.z {
					// catch in case leaves.1 is 0
					s = node;
				}
			} else {
				if let Some(node) = &s.z {
					s = node;
				} else if let Some(node) = &s.o {
					// catch in case leaves.0 is 0
					s = node;
				}
			}
		}

		match s.val.clone() {
			Some(val) => {
				Some(val.clone())
			},
			None => None,
		}
	}
}

fn two(buf: &mut BufReader<std::fs::File>) -> Result<(), bool> {
	let mut ln = String::with_capacity(12);
	let mut node =  Node::new(0);

	while let Ok(ri) = buf.read_line(&mut ln) {
		if ri <= 1 {
			break;
		}
		let val_str = ln.trim().to_string();
		node.push(val_str)?;

		ln.clear();
	}

	let oxygen = u16::from_str_radix(node.find_oxygen().unwrap().as_str(), 2);
	let carbon = u16::from_str_radix(node.find_carbon().unwrap().as_str(), 2);
	println!("({}, {})", oxygen.unwrap(), carbon.unwrap());

	Ok(())
}
