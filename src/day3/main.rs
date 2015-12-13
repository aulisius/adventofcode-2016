use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn calc_houses(mut houses: HashMap<(i32, i32), i32>, input: Vec<char>) -> HashMap<(i32, i32), i32> {

    input.iter().fold((0, 0), |mut loc, &ele| -> (i32, i32) {

        match ele {
            '^' => loc.1 += 1,
            '>' => loc.0 += 1,
            '<' => loc.0 -= 1,
            'v' => loc.1 -= 1,
             _  => (),
        }

        if !houses.contains_key(&loc) {
            houses.insert(loc, 1);
        }

        loc
    });

    houses
}

fn main() {

    let mut input = String::new();

    File::open("input.txt").unwrap().read_to_string(&mut input);

    let input_vec: Vec<_> = input.chars().collect();

    let mut houses = HashMap::new();
    houses.insert((0, 0), 1);

    houses = calc_houses(houses, input_vec);

    // Part 1
    println!("Unique houses: {}", houses.len());

    houses.clear();
    houses.insert((0, 0), 1);

    let mut count: i32 = 0;

    let santa_input: Vec<_> = input.chars()
                                   .filter(|&x| {
                                       count += 1;
                                       count % 2 == 1
                                   })
                                   .collect();

    houses = calc_houses(houses, santa_input);

    count = 0;
    let robo_input: Vec<_> = input.chars()
                                  .filter(|&x| {
                                      count += 1;
                                      count % 2 == 0
                                  })
                                  .collect();

    houses = calc_houses(houses, robo_input);

    // Part 2
    println!("Unique houses visited by Santa and Robo: {}", houses.len());
}
