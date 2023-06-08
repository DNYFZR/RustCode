// Main Shenanigans 
#[allow(dead_code, unused_imports)]
mod modules;

use modules::api::json;
use modules::simulator;
use modules::standard::std_lib as mylib;
use modules::dist::rndm;
use modules::basics::{bsic, vector_sort};
use modules::web::server;

use std::time::Instant;
use ndarray_rand::rand::{Rng, thread_rng, seq::IteratorRandom};
use ndarray::prelude::*;
use cute;
use actix_web::{web, App, HttpServer};

fn main() {
    web_server_main();
}

pub fn basics_main() {
    let x = 8.0;
    let y = 3.0;

    let res_1 = bsic::square_it(x);
    let res_2 = bsic::power_play(x, y);

    if res_1 == res_2 {
        println!("square_it({}) == power_play({}, {})", x, x, y );
    } else {
        println!("No equality");
    }
    
    println!("{}", bsic::recusive_facts(3));

    for i in 0..10 {
        if i <= 8 {
            print!("{}\n", bsic::square_it(i as f64));
        } else {
            print!("{}\n", bsic::power_play(i as f64, 3.0));
        }
    }

    // arrays 
    let arr = [10,20,30];
    println!("first element : {}", arr[0]);
    
    let arr_mapped = arr.map(|arr| arr as f64).map(bsic::square_it);
    print!("Array mapped : {:?}", arr_mapped);

    // ropes
    bsic::find_the_ropes();

    // Arrays & indexing
    bsic::index_array();
    
    let slice_sum = bsic::slice_array_sum(&[10, 20, 30, 40, 88], 3, 4);
    println!("{}", slice_sum);

    println!("{:?}", bsic::slice_array(&[1,2, 3,4, 5,6, 7,8, 9,10], 1, 5));
    println!("{:?}", bsic::slice_index(&[1,2, 3,4, 5,6, 7,8, 9,10], 8));

    // Vectors
    bsic::build_vector();
    bsic::vector_macros();
    vector_sort::sort_int_vec();
    vector_sort::sort_float_vec();
    vector_sort::sort_struct_vec();

    // Iterators
    bsic::iter_outputs(0, 10);
    bsic::iter_sum();
    bsic::iter_windows(&[1,2,3,4,5,67,7,8,7,6,5,4,3,2,11], 3);
    bsic::iter_chunks(&[1,2,3,4,5,67,7,8,7,6,5,4,3,2,11], 5);

}
 
pub fn imports_main() {
    let test_path = "data/test.csv";

    // Environment info
    mylib::env_stuff();

    // Hash map
    println!("dict : {:?}", mylib::hashmap_stuff(16, 256));

    // Read files
    let rows = mylib::read_csv(test_path, ",", false).unwrap();
    let cols = mylib::read_csv(test_path, ",", true).unwrap();

    println!("As rows : {:?} \nAs cols : {:?}", rows[0], &cols[0][0..5]);

 }

pub fn random_values() {
    
    // Uniform numeric
    rndm::random_values();
    rndm::random_dist();

    // Non-uniform dist's
    for _i in 1..3{
        rndm::weibull_dist().unwrap();
    }

    // Mixed type
    rndm::random_tuple();

    // Character methods
    rndm::random_string(30);
    rndm::random_password(20);

}   

// pub fn array_sim_main() {
//     // Sim scale
//     let n_steps = 30;
//     let n_iters = 5;
    
//     // Survival array
//     let probs = vec![1.0, 0.95, 0.96, 0.95, 0.93, 0.92, 0.9, 
//                                 0.88, 0.85, 0.82, 0.8, 0.75, 0.72, 0.7, 
//                                 0.68, 0.65, 0.6, 0.55, 0.5, 0.45, 0.4, 
//                                 0.38, 0.25, 0.1, 0.08, 0.05, 0.01,];
    
//     // Initial states
//     let mut rng = thread_rng();
//     let tmp = vec![2,2,3,4,3,2,4,5,7,8,6,5,3,2,2,4,5,6,7,5,3,2,4,6,6,7];
    
//     let states = cute::c!(tmp.iter().choose(&mut rng).copied().unwrap() , 
//                                     for _i in 0..10 as u64);
    
//     // Run sim
//     let timer = Instant::now();
    
//     let res = simulator::models::array_simulator(states, probs, n_steps);
//     // cute::c![arr_model(states.clone(), probs.clone(), n_steps), for _i in 0..n_iters];
//     let run_time = timer.elapsed(); 
    
//     // println!("{:?}", res);
    
//     println!("Execution completed in {:.2?}", run_time);
//     return res
// }

pub fn micro_sim_main() -> Vec<i32> {
    // config
    let n_steps = 50;
    
    let probs = vec![1.0, 0.95, 0.96, 0.95, 0.93, 0.92, 0.9, 
                                0.88, 0.85, 0.82, 0.8, 0.75, 0.72, 0.7, 
                                0.68, 0.65, 0.6, 0.55, 0.5, 0.45, 0.4, 
                                0.38, 0.25, 0.1, 0.08, 0.05, 0.01,];

    // Create large states array - temp - size would be known at input
    let mut rng = ndarray_rand::rand::thread_rng();
    let states: Vec<i32> = [0; 100e3 as usize].iter().map(|_ : &i32| rng.gen_range(0..25)).collect();
    
    // Simulate
    // let timer = Instant::now();
    // let mut res = ndarray::Array::zeros((50, states.len(), n_steps + 2));
    // res = res.iter().map(|mut x| x = &simulator::models::micro_sim(states, probs, n_steps)).collect();

    // let run_time = timer.elapsed();     
    // println!("Execution completed in {:.2?}", run_time);
    states
}

pub fn simulator_main() {
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
    let res = simulator::models::simulator(states, probs, n_steps, n_iters);

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

pub fn vec_sim_main() -> Vec<Vec<Vec<u64>>> {
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
                                    for _i in 0..100e3 as u64);
    
    // Run sim
    let timer = Instant::now();
    
    let res = cute::c![simulator::models::vec_sim(states.clone(), probs.clone(), n_steps), for _i in 0..n_iters];
    let run_time = timer.elapsed(); 
    
    println!("Execution completed in {:.2?}", run_time);
    println!("{:?}", res);
    return res
}

pub fn vec_comp_main() -> Vec<Vec<Vec<u64>>> {
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
                                    for _i in 0..100e3 as u64);
    
    // Run sim
    let timer = Instant::now();
    
    let res = cute::c![simulator::models::vec_comp(states.clone(), probs.clone(), n_steps), for _i in 0..n_iters];
    let run_time = timer.elapsed(); 
    
    // println!("{:?}", res);
    
    println!("Execution completed in {:.2?}", run_time);
    return res
}

pub fn vec_loop_main() -> Vec<Vec<Vec<u64>>> {
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
    
    let res = cute::c![simulator::models::vec_loop(states.clone(), probs.clone(), n_steps), for _i in 0..n_iters];
    let run_time = timer.elapsed(); 
    
    // println!("{:?}", res);
    
    println!("Execution completed in {:.2?}", run_time);
    return res
}

pub fn web_server_main() {
    // Configure app server
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(server::get_index))
            .route("/gcd", web::post().to(server::post_gcd))
    });

    // Launch app server
    println!("Serving on http://localhost:3000...");
    server.bind("127.0.0.1:3000").expect("Error binding to server address").run().expect("Error running server");
}
