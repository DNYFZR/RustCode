#![allow(unused)] // remove warnings for ununsed vars

use std::any::Any;
use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Initialise vectors
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];

    // Append 
    vec2.push(5);

    // Index 
    println!("1st : {}", vec2[0]);

    let second: &i32 = &vec2[1];

    match vec2.get(1){

        Some(second) => println!("2nd : {}", second),
        None => println!("No second value")
    }

    // change values
    for i in &mut vec2 {
        *i *= 2
    }

    println!("Vector length : {}", vec2.len());

    // remove
    println!("Pop : {:?}", vec2.pop());

}    