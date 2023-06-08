// Discrete Event Sim
use std::time::Instant;
use std::iter::repeat;
use ndarray::prelude::*;
use ndarray_rand::rand::{Rng, thread_rng, seq::IteratorRandom};
use cute;

// pub fn array_simulator(states: Vec<u64>, probs: Vec<f64>, n_steps:usize,) {
//     // initialise sampling
//     let mut rng = thread_rng(); 

//     // base matrix
//     let mut arr: ndarray::ArrayBase<ndarray::OwnedRepr<u64>, ndarray::Dim<[usize; 2]>> = Array::default((states.len(), n_steps + 1));
    
//     // insert base states
//     for mut pair in arr.axis_iter_mut(Axis(0)).enumerate() {
//         pair.1[0] = states[pair.0]; 
//     }

//     // Run sim
//     // if prob @ age > prob @ rand then set next val as age + 1 else 0 
//     for mut iter_row in &mut arr.axis_iter_mut(Axis(0)) {

//         for (col, val) in iter_row.iter().enumerate() {
       
//             if col > 0 && Some(probs[*val as usize]) >= probs.iter().choose(&mut rng).copied()  {
       
//                iter_row[col] = val + 1
//             }
//         }
//     }
//     println!("{:?}", arr);

    
//     // state_mat = cute::c![
//     //     cute::c![
//     //         if i > 0 && Some(probs[arr[i] as usize]) >= probs.iter().choose(&mut rng).copied() {arr[i] + 1 
//     //         } else {0 as u64}, for i in 0..(n_steps + 1)],
//     //         for arr in state_mat];
    
//     // return state_mat.unwrap(); 
// }

pub fn micro_sim (states:Vec<i32>, probs:Vec<f64>, n_steps:usize) -> ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>> {
    let n_steps = n_steps;
    let probs = probs;
    let states = states;

    
    // Create model container  
    let mut result: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>> = ndarray::Array::zeros((states.len(), n_steps + 2));

    // Insert params
    for (n, j) in states.iter().enumerate() {
        // set initial state
        result[[n, 1]] = *j;    
    }

    // Run simulation
    let mut rng = ndarray_rand::rand::thread_rng();
    for mut row in result.rows_mut() {
        for i in 0..(row.len() - 1) {
            if i > 0 && Some(probs[row[i] as usize]) >= probs.iter().choose(&mut rng).copied() { row[i + 1] = row[i] + 1; } else { row[i + 1] = 0; }
        }
    }
    return result
}

pub fn simulator (states:Vec<i32>, probs:Vec<f64>, n_steps:usize, n_iters:usize) -> ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>> {
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
            if i > 0 && Some(probs[row[i] as usize]) >= probs.iter().choose(&mut rng).copied() { row[i + 1] = row[i] + 1; } else { row[i + 1] = 0; }
        }
    }
    return result
}

pub fn vec_sim(states: Vec<u64>, probs: Vec<f64>, n_steps:usize,) -> Vec<Vec<u64>> {
    // initialise iteration vectors
    let mut rng = thread_rng(); 

    // base matrix
    let mut state_mat = vec![];

    // iterate through provided base states
    for i in &states {

        // initial vector
        let mut tmp = vec![0; n_steps + 1];
        tmp[0] =  *i;
    
        // Run sim 
        for j in 0..(tmp.len() - 1) {

            // if prob @ age > prob @ rand then set next val as age + 1 else 0
            if Some(probs[tmp[j] as usize]) >= probs.iter().choose(&mut rng).copied() {
                tmp[j + 1] = tmp[j] + 1;

            } else {
                tmp[j + 1] = 0;
            }
        }
        // push vec to matrix
        state_mat.push(tmp);
    }
    return state_mat; 
}

pub fn vec_comp(states: Vec<u64>, probs: Vec<f64>, n_steps:usize,) -> Vec<Vec<u64>> {
    // initialise sampling
    let mut rng = thread_rng(); 

    // base matrix
    let mut state_mat= cute::c![
        [vec![i], vec![0;n_steps]].concat(), for i in states];

    // Run sim
    // if prob @ age > prob @ rand then set next val as age + 1 else 0 
    state_mat = cute::c![
        cute::c![
            if i > 0 && Some(probs[arr[i] as usize]) >= probs.iter().choose(&mut rng).copied() {arr[i] + 1 
            } else {0 as u64}, for i in 0..(n_steps + 1)],
            for arr in state_mat];
    
    return state_mat; 
}

pub fn vec_loop(states: Vec<u64>, probs: Vec<f64>, n_steps:usize,) -> Vec<Vec<u64>> {
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


