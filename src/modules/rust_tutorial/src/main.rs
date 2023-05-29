#![allow(unused)] // remove warnings for ununsed vars

use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

// Closures - can access variables outside its body - unlike functions
// let var_name = |parameters| -> retrun_type {
//      {BODY}

fn main() {
    let can_vote = |age: i32| {
        age >= 18 // returns true / false
    };

    let mut sampl1 = 5;
    let print_var = ||  println!("sample : {}", sampl1);

    print_var();
    sampl1 = 10;
    print_var();


    println!("Can Vote : {}", can_vote(17));


}