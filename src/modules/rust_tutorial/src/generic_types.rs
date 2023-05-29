#![allow(unused)] // remove warnings for ununsed vars

use std::any::Any;
use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use std::ops::Add;

// for unknown type - define the add trait
fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return  x + y;
}


fn main() {

    // use get_sum_gen
    let a = 5;
    let b = 4.5;
    
    println!("{} + {} = {}", a, b, get_sum_gen(a as f32, b));
}   