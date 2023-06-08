use std::{io::{self, Read}, fs};


fn std_tst() {
    let mut file = io::BufReader::new(fs::File::open("data/test.txt")?);

    let res = file.read_to_string(&mut String::new());

    println!("{:?}", res)
}

fn main() {
    std_tst()
}