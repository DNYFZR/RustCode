// Programming Rust - Chapter 2

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Test values are not zero
    assert!(n !=0 && m != 0);

    // while loop
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    
    n
}

