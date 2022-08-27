use std::collections::HashMap;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut ints = String::new();
    let ints: Vec<i32> = loop {
        println!("Enter a list of integers separated by whitespace");
        stdin
            .read_line(&mut ints)
            .expect("The operating system should be able to read user input");
        match ints
            .split_whitespace()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|int| int.parse())
            .collect()
        {
            Ok(ints) => break ints,
            _ => {
                println!("Invalid format - expected zero or more integers separated by whitespace");
                ints.clear();
                continue;
            }
        }
    };
    match median(&ints) {
        Some(median) => {
            println!("The median of the list truncated to the nearest integer is {median}")
        }
        _ => println!("The list has no median"),
    }
    match mode(&ints) {
        Some(mode) => println!("A mode of the list is {mode}"),
        _ => println!("The list has no mode"),
    };
}

fn median(ints: &[i32]) -> Option<i32> {
    let len = ints.len();
    if len == 0 {
        return None;
    }
    let mut sorted_ints = Vec::from(ints);
    sorted_ints.sort_unstable();
    let middle = len / 2;
    Some(if 2 * middle == len {
        (sorted_ints[middle - 1] + sorted_ints[middle]) / 2
    } else {
        sorted_ints[middle]
    })
}

fn mode(ints: &[i32]) -> Option<i32> {
    let mut counts = HashMap::new();
    for n in ints {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
    }
    let mut result = None;
    let mut max_count = 0;
    for (n, count) in counts {
        if count > max_count {
            result = Some(*n);
            max_count = count;
        }
    }
    result
}
