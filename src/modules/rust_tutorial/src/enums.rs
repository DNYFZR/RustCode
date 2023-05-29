#![allow(unused)] // remove warnings for ununsed vars

use std::any::Any;
use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {

    enum Day {
        Mon,
        Tue,
        Wed,
        Thu,
        Fri,
        Sat,
        Sun
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Sat | Day::Sun => true,
                _ => false,
            }
        }
    }

    let today:Day = Day::Mon;
    match today {
        Day::Mon => println!("Not another Monday"),
        Day::Tue => println!("Two for"),
        Day::Wed => println!("Half way"),
        Day::Thu => println!("Last stretch"),
        Day::Fri => println!("Don't deploy"),
        Day::Sat | Day::Sun => println!("Weekend baby"),
    }

    println!("Is today the weekend :  {}", today.is_weekend());

}    
