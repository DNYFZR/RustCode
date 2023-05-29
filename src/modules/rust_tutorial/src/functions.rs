#![allow(unused)] // remove warnings for ununsed vars

use std::any::Any;
use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn get_sum(x: i32, y: i32) -> i32 {
    println!("Hey bro...✌🏻");
    println!("{} + {} = {}", x, y, x+y);

    x + y
}

fn get_two (x: i32) -> (i32, i32) {
    return (x+1, x+2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
}

fn main() {
    let x = get_sum(8, 64);

    let (val_1, val_2) = get_two(x);
    println!("val_1 = {} , val_2 = {}", val_1, val_2);

    // sum values in list
    let num_list = vec![1,2,3,4,5,];

    println!("Sum of list = {}", sum_list(&num_list))
}    
