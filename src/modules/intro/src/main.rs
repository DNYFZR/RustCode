// Chapter 2 - a tour of rust

use std::str::FromStr;
use std::env;

// Rust functions
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Test values are not zero
    assert!(n !=0 && m != 0);

    // while loop
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // return greatest common divisor
    return n
}

// Unit tests
#[test] // Attribute notation - test will skip normal compilations & only run in test mode
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
}


fn main() {
    
    // Initialise vector - similar to Python / Java arrays
    let mut nums = Vec::new();

    // Parse CMD string arguments (iterator) to numeric
    for arg in env::args().skip(1){ // env::args() first argument (index=0) is program name - skip(1) element
        nums.push(u64::from_str(&arg).expect("Error parsing argument..."));
    }

    // Check user input & error if invalid
    if nums.len() == 0 {
        eprint!("Usage: gcd NUMBER...");
        std::process::exit(1);
    }

    // Run method on input
    let mut d = nums[0];
    for m in &nums[1..] {
        d = gcd(d, *m);
    }
        
    println!("Greatest divisor of {:?} is {}", nums, d);
}

// To run main : cargo run 42 56
