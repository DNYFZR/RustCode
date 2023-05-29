// Rust by Example Study Code

// Algorithms - random numbers

use rand::Rng;

fn main () {
    // Uniform numeric
    random_values();
    random_dist();

    // Non-uniform dist's
    for _i in 1..3{
        weibull_dist().unwrap();
    }

    // Mixed type
    random_tuple();

    // Character methods
    random_string(30);
    random_password(20);
}

fn random_values() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2:u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

// Distributions
use rand::distributions::{Distribution, Uniform};

fn random_dist() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("{}", throw);

        if throw == 6 {
            break;
        }
    }
}

// Non-uniform 
use rand_distr::{Weibull, WeibullError};

fn weibull_dist() -> Result<(), WeibullError> {
    let mut rng = rand::thread_rng();
    
    let normal = Weibull::new(25.0, 3.0)?;
    let v = normal.sample(&mut rng);

    println!("{}", v);
    Ok(())
}

// Random values of custom type
use rand::distributions::Standard;

#[derive(Debug)]
struct Point {
    x: i32, 
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point { x: rand_x, y: rand_y }
    }
}

fn random_tuple() {
    let mut rng = rand::thread_rng();
    
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();

    println!("Random Tuple : {:?}", rand_tuple);
    println!("Random Point : {:?}", rand_point);
}

// Character distributions
use rand::distributions::Alphanumeric;

fn random_string(len:usize) {
    let rand_str: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    println!("{}", rand_str);
}

fn random_password(len:usize) {
    let mut rng = rand::thread_rng();
    
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";   
    
    let password: String = (0..len).map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    }).collect();

    println!("{:?}", password)
}