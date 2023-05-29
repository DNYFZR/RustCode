#![allow(unused)] // remove warnings for ununsed vars

use std::any::Any;
use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Stack : store of values - last in / first out - fixed size
// Heap : request x amount of space - OS finds space and provides pointer (address)

// Rules
//  - each value has a variable - the owner
//  - there can be only one owner of a value at a time
//  - when the owner goes out of scope, the value is removed from the system


fn print_string(x: String) {
    println!("A string : {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A String {}", x);
    x
}

fn change_string(name: &mut String) -> String{
    name.push_str(" is happy");
    println!("Message : {}", name);
    return name.to_string();
}

fn main() {
    let str_1 = String::from("World");
    let str_2 = str_1.clone(); 

    print_string(str_2.to_string()); // borrow error without clone in below print

    let str_3 = print_return_str(str_2.clone());
    println!("Third one : {}", str_3);

    let mut str_4 = str_1;
    let mut str_5 = change_string(&mut str_4);
    
    change_string(&mut str_5);
    
}    
