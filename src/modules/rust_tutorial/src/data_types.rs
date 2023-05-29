#![allow(unused)] // remove warnings for ununsed vars

use std::any::Any;
use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // data type limits
    println!("u32: {} - {}", u32::MIN, u32::MAX);
    println!("f32: {} - {}", f32::MIN, f32::MAX);
    println!("usize: {} - {}", usize::MIN, usize::MAX);
    

    // Strings
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str("word");

    for word in st1.split_whitespace(){
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    // Random strings
    let st3 = String::from("ABCDEFGHIJKLMOPQRSTUVXYZ123456789");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();

    for char in v1 {
        println!("{}", char);
    }

    let st4 = "Random String";
    let mut st5 = st4.to_string();

    println!("{}", st5);

    // string to bytes array
    let byte_array = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length : {}", st6.len());

    st5.clear();

    // combine strings
    let st6 = String::from("just some");
    let st7 = String::from("words");
    let st8 = st6 + &st7; // st6 has been released and its value reassinged to st8

    for char in st8.as_bytes(){
        println!("{}", char);
    }

    // casting
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;

    let int3_u32: u32 = int_u8 as u32 + int2_u8 as u32;
    println!("cast value : {}", int3_u32)

}    
