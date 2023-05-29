#![allow(unused)] // remove warnings for ununsed vars

use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    
    // control flow
    let age = 8;

    if (age >= 1) && (age <=18) {
        println!("something important");
    } else if (age == 21) || (age == 50) {
        println!("something important");
    } else if age >= 65 {
        println!("something importatn");
    } else {
        println!("Sorry bro...");
    }

    // ternary op
    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };

    println!("can vote: {}", can_vote);

    // match 
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important"), // = in range includes last value (18)
        21 | 50 => println!("Important"),
        65..=i32::MAX => println!("Important"),
        _ => println!("Not Important"),
    };

    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Cannot Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("Can Vote for First Time")
    }

    // Arrays - unifor data type and fixed size
    let arr_1 = [1,2,3,4];
    
    println!("first {}", arr_1[0]);
    println!("length {}", arr_1.len());

    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;

    // Iterations
    let run_loop = "for";

    match run_loop {
        
        "loop" => loop{
            if arr_2[loop_idx] % 2 == 0 {
                // increment index and move to next loop
                loop_idx += 1;
                continue;
            }
            if arr_2[loop_idx] == 9 {
                // end loop
                break;
            }
            println!("Val {}", arr_2[loop_idx]);
            loop_idx += 1;
        },

        "while" => while loop_idx < arr_2.len() {
            println!("Array : {}", arr_2[loop_idx]);
            loop_idx += 1;
        },
        
        "for" => for val in arr_2.iter() {
            println!("Val : {}", val);
        },
        _ => ()      
    };



}
