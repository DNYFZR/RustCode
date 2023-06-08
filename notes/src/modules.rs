#![allow(unused)] // remove warnings for ununsed vars
use std;
use rand;
use crate::pizza_place::order_food;

mod pizza_place {

    #[derive(Debug)]
    pub struct Pizza {
        pub dough: String,
        pub sauce: String,
        pub cheese: String,
        pub toppings: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza { 
                dough: String::from("regular dough"), 
                sauce: String::from("tomato sauce"),
                cheese: String::from("mozzarella"),
                toppings: String::from(topping), 
            }
        }
    }

    pub mod help_customer{
        use core::time;
        use std::{process::Command, thread::sleep};

        use rand::Rng;

        fn seated_at_table() -> i32{
            let table = rand::thread_rng().gen_range(1..=21);
            println!("Customer seated at table {}", table);
            
            table
        }
        fn serve_customer(customer_order: super::Pizza) -> bool{
            // Kitchen time
            let cook_time = 20;
            println!("Order up in {}", cook_time);
            
            std::thread::sleep(std::time::Duration::from_secs_f32(cook_time as f32));
            

            // Order served
            println!(
                "Customer served : {} with {}, {} and {}", 
                customer_order.dough, 
                customer_order.cheese, 
                customer_order.sauce, 
                customer_order.toppings
            );

            true
        }
        pub fn take_order(){
            // seat customer
            let customer_table = seated_at_table();

            // take order
            let customer_order: super::Pizza = super::Pizza::lunch("veggies");
            println!("Table {}, Order: {:?}", customer_table, customer_order);

            // serve order
            serve_customer(customer_order);
        }
    }

    pub fn order_food(){
        crate::pizza_place::help_customer::take_order()
    }
}



fn main() {
    order_food()
}  