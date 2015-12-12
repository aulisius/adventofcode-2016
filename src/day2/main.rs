use std::io::prelude::*;
use std::fs::File;

fn calc_wrapping_paper(dimen: &[i32]) -> i32 {

	let l_w = dimen[0] * dimen[1];
	let w_h = dimen[1] * dimen[2];
	let h_l = dimen[2] * dimen[0];

	let smallest_area: i32 = [l_w, w_h, h_l].iter().fold(std::i32::MAX, |n, &i| if i < n {i} else {n});
	
	let area: i32 = 2* (l_w + w_h + h_l);

	area + smallest_area
}

fn calc_ribbon(dimen: &mut[i32]) -> i32 {

	dimen.sort_by(|a, b| a.cmp(b));

	let perimeter: i32 = 2 * (dimen[0] + dimen[1]);

	let volume = dimen[0] * dimen[1] * dimen[2];
	
	perimeter + volume
}

fn main() {


	let mut str = String::new();

	File::open("input.txt").unwrap().read_to_string(&mut str);

	let total = str.split_whitespace().fold((0, 0), |mut acc, elem| -> (i32, i32) {

		let mut value: Vec<_> = elem.split('x').map(|c| c.parse::<i32>().unwrap()).collect();

		acc.0 = acc.0 + calc_wrapping_paper(value.as_ref());
		
		acc.1 = acc.1 + calc_ribbon(value.as_mut());

		acc
	});

	// Part 1
	println!("Total amount of wrapping paper: {} in sq. feet", total.0);

	// Part 2 
	println!("Total amount of ribbon: {} in feet", total.1);

}
