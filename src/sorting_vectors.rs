// Rust by Example Study Code

// 1.2 Algorithms - Sorting Vectors

fn main () {
    sort_int_vec();
    sort_float_vec();
    sort_struct_vec();
}

// Sort vectors
fn sort_int_vec() {
    let mut v = vec![1,5,10,2,6,4,19,12];
    
    v.sort();

    assert_eq!(v, vec![1,2,4,5,6,10,12,19]);
}

fn sort_float_vec() {
    let mut v = vec![0.1, 0.9, 0.5, 0.4, 0.8, 0.2];

    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(v, vec![0.1, 0.2, 0.4, 0.5, 0.8, 0.9]);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name:String, age:u32) -> Self {
        Person{
            name,
            age,
        }
    }
}

fn sort_struct_vec() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // Sort by derived natural order
    people.sort();

    assert_eq!(people,vec![
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
        Person::new("Zoe".to_string(), 25),]);
    
    // Sort by age
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(people,vec![
        Person::new("Al".to_string(), 60),
        Person::new("Zoe".to_string(), 25),
        Person::new("John".to_string(), 1),]);
    
}