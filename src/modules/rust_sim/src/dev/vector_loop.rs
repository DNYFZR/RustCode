// Discrete Event Sim
use std::time::Instant;
use ndarray_rand::rand::{thread_rng, seq::IteratorRandom};
use cute;

pub fn vec_model(states: Vec<u64>, probs: Vec<f64>, n_steps:usize,) -> Vec<Vec<u64>> {
    // initialise sampling
    let mut rng = thread_rng(); 

    // base matrix
    let mut state_mat = vec![];

    // iterate through provided base states
    for i in &states {

        // initial vector
        let mut tmp = vec![*i];
        
        // Run sim

        // if prob @ age > prob @ rand then set next val as age + 1 else 0 
        cute::c![ if i > 0 && Some(probs[tmp[i] as usize]) >= probs.iter().choose(&mut rng).copied() {
                tmp.push(tmp[i] + 1) } else {tmp.push(0 as u64)}, for i in 0..(n_steps + 1)]; 

        
        // push vec to matrix
        state_mat.push(tmp);
    }
    return state_mat; 
}

pub fn main() -> Vec<Vec<Vec<u64>>> {
    // Sim scale
    let n_steps = 30;
    let n_iters = 500;
    
    // Survival array
    let probs = vec![1.0, 0.95, 0.96, 0.95, 0.93, 0.92, 0.9, 
                                0.88, 0.85, 0.82, 0.8, 0.75, 0.72, 0.7, 
                                0.68, 0.65, 0.6, 0.55, 0.5, 0.45, 0.4, 
                                0.38, 0.25, 0.1, 0.08, 0.05, 0.01,];
    
    // Initial states
    let mut rng = thread_rng();
    let tmp = vec![2,2,3,4,3,2,4,5,7,8,6,5,3,2,2,4,5,6,7,5,3,2,4,6,6,7];
    
    let states = cute::c!(tmp.iter().choose(&mut rng).copied().unwrap() , 
                                    for _i in 0..10e3 as u64);
    
    // Run sim
    let timer = Instant::now();
    
    let res = cute::c![vec_model(states.clone(), probs.clone(), n_steps), for _i in 0..n_iters];
    let run_time = timer.elapsed(); 
    
    // println!("{:?}", res);
    
    println!("Execution completed in {:.2?}", run_time);
    return res
}
