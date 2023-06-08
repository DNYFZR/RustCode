// Algorithms - random numbers (Rust by Example)

use rand::Rng;

pub fn random_values() {
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

pub fn random_dist() {
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

pub fn weibull_dist() -> Result<(), WeibullError> {
    let mut rng = rand::thread_rng();
    
    let normal = Weibull::new(25.0, 3.0)?;
    let v = normal.sample(&mut rng);

    println!("{}", v);
    Ok(())
}

// Random values of custom type
use rand::distributions::Standard;

#[derive(Debug)]
pub struct Point {
    x: i32, 
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point { x: rand_x, y: rand_y }
    }
}

pub fn random_tuple() {
    let mut rng = rand::thread_rng();
    
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();

    println!("Random Tuple : {:?}", rand_tuple);
    println!("Random Point : {:?}", rand_point);
}

// Character distributions
use rand::distributions::Alphanumeric;

pub fn random_string(len:usize) {
    let rand_str: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    println!("{}", rand_str);
}

pub fn random_password(len:usize) {
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