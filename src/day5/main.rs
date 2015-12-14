extern crate pcre;

use std::io::prelude::*;
use std::fs::File;

use pcre::Pcre;

fn main() {

    let mut input = String::new();

    File::open("input.txt").unwrap().read_to_string(&mut input);
    
    let v_re = Pcre::compile("(:?([aeiou].*){3})").unwrap();
    let p_re = Pcre::compile("(:?(ab|cd|pq|xy))").unwrap();
    let d_re = Pcre::compile("(:?((\w)\1))").unwrap();

    let nice_res: Vec<_> = input.split_whitespace()
                           .filter(|&ele| -> bool {
                               v_re.is_match(ele) && p_re.is_match(ele) && d_re.is_match(ele)  
                           })
                           .collect();

    //Part 1
    println!("No.of nice strings: {}", nice_res.len());

    let t_re = Pcre::complie("(:?((\w).\1))").unwrap(); 
    let dp_re = Pcre::complie("(:?((\w{2}).*\1))").unwrap(); 

    let nicer_res: Vec<_> = input.split_whitespace()
                           		.filter(|&ele| { t_re.is_match(ele) && dp_re.is_match(ele) })
		                        .collect();

	//Part 2
	println!("No.of nicer strings: {}", nicer_res.len());

}
