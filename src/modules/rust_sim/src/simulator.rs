// Discrete Event Simulator (in development)

use std::time::Instant;
// use std::fs::File;
// use csv::WriterBuilder;
// use ndarray_csv::Array2Writer;
use ndarray::{self, s};
use ndarray_rand::rand::seq::IteratorRandom;
use ndarray_rand::rand::Rng;

pub fn run_sim (states:Vec<i32>, probs:Vec<f64>, n_steps:usize, n_iters:usize) -> ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>> {
    let n_steps = n_steps;
    let n_iters = n_iters;
    let probs = probs;
    let states = states;

    
    // Create model container  
    let mut init_mat: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 3]>> = ndarray::Array::zeros((n_iters, states.len(), n_steps + 2));

    // Initialise params
    for (i, mut row) in init_mat.axis_iter_mut(ndarray::Axis(0)).enumerate() {
        for (n, j) in states.iter().enumerate() {
            // set iteration number            
            row[[n, 0]] = i as i32;

            // set initial state
            row[[n, 1]] = *j;    
        }
    }

    // Reshape to 2D array
    let mut result = init_mat.to_shape((n_iters * states.len(), n_steps + 2)).unwrap().to_owned();


    // Run simulation
    let mut rng = ndarray_rand::rand::thread_rng();
    for mut row in result.rows_mut() {
        for i in 0..(row.len() - 1) {
            if i > 0 && Some(probs[row[i] as usize]) >= probs.iter().choose(&mut rng).copied() { row[(i + 1)] = row[i] + 1; } else { row[(i + 1)] = 0; }
        }
    }
    return result
}



pub fn main() {
    // config
    let n_steps = 50;
    let n_iters = 500;

    let probs = vec![1.0, 0.95, 0.96, 0.95, 0.93, 0.92, 0.9, 
                                0.88, 0.85, 0.82, 0.8, 0.75, 0.72, 0.7, 
                                0.68, 0.65, 0.6, 0.55, 0.5, 0.45, 0.4, 
                                0.38, 0.25, 0.1, 0.08, 0.05, 0.01,];

    // Create large states array - temp - size would be known at input
    let mut rng = ndarray_rand::rand::thread_rng();
    let states: Vec<i32> = [0; 100e3 as usize].iter().map(|_ : &i32| rng.gen_range(0..25)).collect();
    
    // Simulate
    let timer = Instant::now();    
    let res = run_sim(states, probs, n_steps, n_iters);

    let run_time = timer.elapsed();     
    println!("Execution completed in {:.2?}", run_time);
    println!("{:?}", res.slice(s![1..10, ..]));

    // Save test
    // {
    //     let file = File::create("test.csv")?;
    //     let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
    //     writer.serialize_array2(result)?;
    // }
}