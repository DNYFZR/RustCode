#![allow(unused)] // remove warnings for ununsed vars

use std::any::Any;
use std::io;
use std::str::FromStr; 
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// constant
const PI: f32 = 3.141592;

// Define a custom data structure
#[derive(Debug)]
struct Customer {
    name: String, 
    address: String,
    balance: f32,
}

#[derive(Debug)]
struct Rectangle {
    length: f32,
    height: f32,
}

struct Circle {
    diameter: f32,
}

trait Shape {
    fn new(length: f32, width:f32) -> Self;
    fn area(&self) -> f32; 
}

impl Shape for Rectangle {
    fn new(length:f32, height:f32) -> Rectangle {
        return Rectangle{length, height};
    }

    fn area(&self) -> f32 {
        return self.length * self.height;
    }
}

impl Shape for Circle {
    fn new(diameter: f32, width:f32) -> Circle {
        return Circle{diameter};
    }

    fn area(&self) -> f32 {
        return 0.25 * PI * self.diameter.powf(2.0);
    }
}

fn main() {

    // Initialise struct
    let mut bob = Customer{
        name: String::from("Bob Smith"),
        address: String::from("888 Main St"),
        balance: 500.00
    }; 

    println!("{:?}", bob);

    // Update struct 
    bob.address = String::from("100 Charming Avenue");
    bob.balance += 1000.00; 

    println!("{:?}", bob);

    let rec = Rectangle{
        length: 4.0,
        height: 10.5
    };

    println!("Rectangle Area : {}", rec.area());

    let circ = Circle{diameter: 0.5};
    
    println!("Circle Area : {}", circ.area());

}