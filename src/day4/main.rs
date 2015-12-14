

extern crate crypto;

use std::fmt;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {

	let mut t = Md5::new();

	let input = "bgvyzdsv";

	let mut num: i32 = 1;

	loop {

		let str =  fmt::format(format_args!("{}{}", input, num));

		t.input_str(&str);

		let result = t.result_str();

		//Part 1
		if result.starts_with("00000") { println!("{}", num); }

		//Part 2
		if result.starts_with("000000") { break; }

		num += 1;

		t.reset();

	}

	println!("{}", num);


}
