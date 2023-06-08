#![allow(unused)] // remove warnings for ununsed vars

use std::any::Any;
use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // tuples
    let my_tuple = (47, "Agent".to_string(), 50_000.00 );
    
    println!("Name {}", my_tuple.1);

    let (v1, v2, v3) = my_tuple;
    println!("Number {}", v1);


}    
