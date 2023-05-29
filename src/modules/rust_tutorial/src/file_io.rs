#![allow(unused)] // remove warnings for ununsed vars

use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;


fn main() {
    let path = "data/lines.txt";
    let output = File::create(path);

    // Result has two variants : Ok and Err
    // enum Result<T, E> { Ok(T), Err(E) }
    // where T is the data type of the value, and E is the error type
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("File not created : {:?}", error);
        }
    };

    write!(output, "Just a test...").expect("Could not write to file...");

    let input = File::open(path).unwrap(); // unwrap to extract result
    let buffered = BufReader::new(input);

    for line in buffered.lines(){
        println!("{}", line.unwrap());
    }


    // catch specific errors
    let pth = "data/rand.txt";
    let output2 = File::create(pth);
   
    let mut output2 = match output2 {
        
        Ok(file) => file,
        
        Err(error) => match error.kind() {
        
            ErrorKind::NotFound => match File::create(pth){
                Ok(fc) => fc,
                Err(e) => panic!("Cannot create file : {:?}", error),
            },

            _other_errors => panic!("Problem opening file : {:?}", error),
            

        }
    };
}