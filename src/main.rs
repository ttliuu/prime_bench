//! A simple benchmark program that computes
//! 10_000_000th prime number

use std::time::SystemTime;
use std::{io, thread};

const PRIME_COUNT: usize = 5_000_000;

fn is_prime(x: u32, prime_vec: &[u32]) -> bool {
    prime_vec.iter().take_while(|&&p| p * p <= x).all(|&p| x % p != 0)
}

fn main() {
    let f = || {
        let mut prime_vec = Vec::with_capacity(PRIME_COUNT);
        prime_vec.push(2);
        let mut num_to_check = 3;
        while prime_vec.len() < PRIME_COUNT {
            if is_prime(num_to_check, &prime_vec) {
                prime_vec.push(num_to_check);
            }
            num_to_check += 2; // Increment by 2 to skip even numbers
        }
        // println!("Count: {}", prime_vec.last().unwrap());
    };
    let mut v = Vec::new();
    println!("How many threads?");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let thread_num: u32 = s.trim().parse().expect("Not a number!");
    println!("Running benchmark with {thread_num} thread{}.", if thread_num == 1 {""} else {"s"});
    let start = SystemTime::now();
    for _ in 0..thread_num {
        v.push(thread::spawn(f));
    }
    for handler in v {
        handler.join().unwrap();
    }
    let stop = SystemTime::now();
    let time = stop.duration_since(start).unwrap();

    println!("total time: {}s", time.as_secs_f32());

    println!("avg. time per thread: {}s", time.as_secs_f32() / thread_num as f32);
}
