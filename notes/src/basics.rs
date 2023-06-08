#![allow(unused)] // remove warnings for ununsed vars

use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Basic IO
    println!("What id your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("didn't receive input");

    println!("Hello {}! {}", name.trim_end(), greeting);

    // constant values
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;

    // shadowing a value - two var with same name and diff data types
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("age wasn't assigned a number");

    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    // compiler will ignore
    let _ignore_this = true;

    // strings
    let string_val = "a string";
    let char_val = 'a'; 

    // precision
    let num_1: f32 = 1.123456789123456;
    let num_2: f64 = 0.123456789123456;

    println!("f32 precision: {}", num_1 + num_1);
    println!("f64 precision: {}", num_2 + num_2);

    // math
    let mut num_3 = 1.90872983;
    num_3 += num_3 ;

    // Random numbers
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("random: {}", random_num);

}