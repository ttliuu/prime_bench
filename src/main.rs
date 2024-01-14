//! A simple benchmark program that computes 
//! 10_000_000th prime number

use std::time::SystemTime;

macro_rules! PRIME_COUNT {
    () => {
        10_000_000
    };
}

fn is_prime(x: u32, prime_vec: &[u32]) -> bool {
    prime_vec.iter().take_while(|&&p| p * p <= x).all(|&p| x % p != 0)
}

fn main() {
    let mut prime_vec = Vec::with_capacity(PRIME_COUNT!());
    prime_vec.push(2);
    let mut num_to_check = 3;
    let start = SystemTime::now();
    while prime_vec.len() < PRIME_COUNT!() {
        if is_prime(num_to_check, &prime_vec) {
            prime_vec.push(num_to_check);
        }
        num_to_check += 2; // Increment by 2 to skip even numbers
    }
    let stop = SystemTime::now();
    let time = stop.duration_since(start).unwrap();

    println!("{}", prime_vec.last().unwrap());
    println!("time: {}s", time.as_secs_f32());
}
