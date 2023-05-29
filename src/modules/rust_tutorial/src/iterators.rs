#![allow(unused)] // remove warnings for ununsed vars

use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;


fn main() {
    let mut arr = [1,2,3,4];

    // iterate through borrowed ref
    for val in arr.iter(){
        println!("{}", val);
    }

    // consume the collection - no longer useable 
    for val in arr.into_iter(){
        println!("{}", val);
    }

    // create iterator
    let mut iter1 =  arr.iter();
    println!("{:?}", iter1.next());

    println!("{:?}", arr);
}