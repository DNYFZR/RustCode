// Rust study - basics


fn square_it(x: f64) -> f64 {
    return f64::powf(x, 2.0);
}

fn power_play(x:f64, y:f64) -> f64 {
    return f64::powf(x, y);
}

fn recusive_facts(x: u64) -> u64 {
    if x == 0 {1}
    else{ x * recusive_facts(x - 1)}
}

fn main() {
    let x = 8.0;
    let y = 3.0;

    let res_1 = square_it(x);
    let res_2 = power_play(x, y);

    if res_1 == res_2 {
        println!("square_it({}) == power_play({}, {})", x, x, y );
    } else {
        println!("No equality");
    }
    
    println!("{}", recusive_facts(3));

    for i in 0..10 {
        if i <= 8 {
            print!("{}\n", square_it(i as f64));
        } else {
            print!("{}\n", power_play(i as f64, 3.0));
        }
    }

    // arrays 
    let arr = [10,20,30];
    println!("first element : {}", arr[0]);
    
    let mut arr_mapped = arr.map(|arr| as f64).map(square_it);
    print!("Array mapped : {:?}", arr_mapped);

}
