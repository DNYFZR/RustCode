#![allow(unused)] // remove warnings for ununsed vars

use std::any::Any;
use std::io; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Hash Maps - key-value stores
use std::collections::HashMap;

fn main() {
    let mut heroes = HashMap::new();

    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Iron Man", "Tony Stark");
    heroes.insert("Captain America", "Steve Rodgers");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);

    }

    println!("Length : {}", heroes.len());

    if heroes.contains_key("Batman"){
        let the_batman = heroes.get(&"Batman");

        match the_batman {
            Some(x) => println!("I'm Batman"),
            None => println!("I'm not Batman")
        }
    }
}