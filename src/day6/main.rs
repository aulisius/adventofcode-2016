use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

/*
*	I modified the input file to the form 
*    option_start.x,start.y,end.x,end.y
*	Then I split the line at '_' and parse the two rows separately.
*/

fn main() {

	let mut input = String::new();

	File::open("input.txt").unwrap().read_to_string(&mut input);

	let lines: Vec<_> = input.split_whitespace().collect();

	let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

	for l in lines {

		let r: Vec<_> = l.split('_').collect();

		let option: i32 = r[0].parse::<i32>().unwrap();	

		let val: Vec<i32> = r[1].split(',').map(|c| c.parse::<i32>().unwrap()).collect();

		for i in val[0]..val[2]+1 {
			for j in val[1]..val[3]+1 {
				
				let loc = (i, j);
							
				if !grid.contains_key(&loc) {
					grid.insert(loc, 0);
				}
				
				match option {
					0 => { 	
							if let Some(x) = grid.get_mut(&loc) {
								*x += 1;
							}
					},
					1 => {
					
							if let Some(x) = grid.get_mut(&loc) {
								*x += if *x > 0 {-1} else {0};
							}
					},
					2 => {
							//Depending on which part of the question you are attempting, you might need to comment out the other part.
							//Part 1 
							if let Some(x) = grid.get_mut(&loc) {
								*x = if *x > 0 {0} else {1};
							}
							
							// Part 2 
							if let Some(x) = grid.get_mut(&loc) {
								*x += 2;
							}
					},
					_ => {}
				}
			}
		}

	}

	
	let mut sum = 0;
	
	for v in grid.values() {
		sum = sum + v;
	}

	grid.clear();
	grid.shrink_to_fit();

	//Part 1 
	println!("No.of lights lit: {}", sum);
	
	//Part 2
	println!("The total brightness: {}", sum);

}
