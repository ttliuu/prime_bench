
fn is_prime(x: u32, prime_vec: &Vec<u32>) -> bool {
    let mut pos = 0;
    loop {
        if prime_vec[pos] * prime_vec[pos] > x {
            break;
        } else if x % prime_vec[pos] == 0 {
            return false;
        }
        pos += 1;
    }
    true
}


fn main() {
    let mut prime_vec = Vec::new();
    prime_vec.push(2);

    let mut num_to_check = 3;
    loop {

        if is_prime(num_to_check, &prime_vec) {
            prime_vec.push(num_to_check.clone());
        }

        num_to_check += 1;



        if prime_vec.len() == 10_000_000 {
            break;
        }
    }
    println!("{}", prime_vec[prime_vec.len() - 1]);


}