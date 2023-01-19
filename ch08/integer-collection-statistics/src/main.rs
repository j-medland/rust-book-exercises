// https://rust-book.cs.brown.edu/ch08-03-hash-maps.html
// Summary Exercise Part 1
// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list

// Note: I am going to allow for multi-modal data and for an empty set, return an empty Vec

use std::collections::HashMap;

fn main() {

    // let input :Vec<i32>= Vec::new();
    let input = vec![2, 1, 1, 4, 5, 5, 6, 7];

    match median(&input[..]) {
        Some(median) => println!("Median: {}", median),
        None => println!("Unable to calculate Median"),
    };

    println!("Modes {:?}", mode(&input[..]));
}

fn median(input: &[i32]) -> Option<i32> {
    let mut v = Vec::from(input);

    v.sort();

    match (v.is_empty(), v.len() % 2 == 0) {
        // ( is_empty, length_is_even)
        (true, _) => None,
        (false, true) => {
            // average values either side of non-integer index
            let left = v[(v.len() - 1) / 2];
            let right = v[(v.len() + 1) / 2];
            Some((left + right) / 2)
        }
        (false, false) => Some(v[v.len() / 2]),
    }
}

fn mode(input: &[i32]) -> Vec<i32> {
    let mut counts = HashMap::new();

    // build map of (value, count)
    let mut max_count = 0;

    for v in input {
        let count = counts.entry(v).or_insert(0);
        *count += 1;
        // keep track of max_count
        if *count > max_count {
            max_count = *count;
        }
    }

    let mut modes = Vec::new();

    // copy the values from the counts (map) into the modes vec
    // if their count == max_count
    for (k, v) in counts {
        if v == max_count {
            modes.push(*k);
        }
    }
    return modes;
}
