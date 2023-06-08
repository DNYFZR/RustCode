// Basics

use std::env::{args, Args};
use std::ops::Not; // apparnetly better than using the ! version 

pub fn square_it(x: f64) -> f64 {
    return f64::powf(x, 2.0);
}

pub fn power_play(x:f64, y:f64) -> f64 {
    return f64::powf(x, y);
}

pub fn recusive_facts(x: u64) -> u64 {
    if x == 0 {1}
    else{ x * recusive_facts(x - 1)}
}

pub fn find_the_ropes() -> f32 {
    let mut env_args: Args = args();
    
    // let exe_path = env_args.nth(0).unwrap();
    // println!('{:#?}', exe_path);

    let env_first = env_args.nth(1).unwrap();
    let operator = env_args.nth(0).unwrap().chars().next().unwrap();
    let env_two = env_args.nth(0).unwrap();
    println!("{} {} {}", env_first, operator, env_two);

    let env_first_no = env_first.parse().unwrap();
    let env_second_no = env_first.parse().unwrap();

    let result = operate(operator, env_first_no, env_second_no);
    result
}

pub fn operate(operator:char, n_first: f32, n_second:f32) -> f32 {
    if operator == '+'{n_first + n_second
    } else if operator == '-' {n_first - n_second
    } else if operator == '/' {n_first / n_second
    } else if operator == '*' { n_first * n_second
    } else {0.0
    }
}

// Arrays & Indexes
pub fn index_array() {
    let arr = [10,20,30,40];
    let fst = arr[0];
    println!("First index = {}", fst);

    for i in 0..4 {
        println!("arr[{}] = {}", i, arr[i])
    }

    println!("array length = {}", arr.len());
}

pub fn slice_array_sum(values: &[i32], from_idx:i32, to_idx:i32) -> i32 { // input = slice of i32
    let mut res = 0;

    for i in from_idx..to_idx + 1 {
        res +=  values[i as usize]
    }
    res
}

pub fn slice_array<'a>(values: &[i32], from_idx:i32, to_idx:i32) -> &[i32] {
    &values[from_idx as usize..to_idx as usize]
}

pub fn slice_index(vals: &[i32], idx:i32) -> &i32 {
    vals.get(idx as usize).unwrap()
}

// Vectors
pub fn build_vector() {
    let mut v = Vec::new(); // or vec![]
    v.push(10);
    v.push(20);
    v.push(30);

    println!("Vector: {:?}", v);
    println!("First = {}", v[0]);

}

pub fn vector_macros() {
    let v = vec![10, 20, 40, 50];
    println!("original : {:?}", v);
    
    let mut v_new = Vec::new();
    v_new.push(
        v.iter()
        .filter(|x| x > &&(v.clone().into_iter().sum::<i32>() / v.len() as i32))
        
    );

    println!("new : {:?}", v_new);
}

// Iterators
pub fn iter_outputs(from_idx:i32, to_idx:i32) {
    let mut iter = from_idx..to_idx;
    
    while iter.is_empty().not() {
        let val = iter.next().unwrap(); 
        assert!(val >= from_idx && val <= to_idx);
        
    };

    assert_eq!(iter.next(), None);
}

pub fn iter_sum() {
    let sum: i32 = (0..5).sum();
    println!("{}", sum);

    let sum: i64 = [88, 99, 101].iter().sum();
    println!("{}", sum);

}

// overlappig windows of an array
pub fn iter_windows(arr: &[i32], window_size: i32) {

    for wndow in arr.windows(window_size as usize) {
        println!("{:?}", wndow);
    }
}

// chunks of an array - no overlap
pub fn iter_chunks(arr:&[i32], chunk_size: i32) { 
    
    for cnk in arr.chunks(chunk_size as usize) {
        println!("{:?}", cnk);
    }
}