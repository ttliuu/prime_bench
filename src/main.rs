//! A simple benchmark program that computes 
//! 10_000_000th prime number

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
    while prime_vec.len() < PRIME_COUNT!() {
        if is_prime(num_to_check, &prime_vec) {
            prime_vec.push(num_to_check);
        }
        num_to_check += 2; // Increment by 2 to skip even numbers
    }

    println!("{}", prime_vec.last().unwrap());
}
