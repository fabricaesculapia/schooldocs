// This code generates a random number between 1 and 100 using the rand crate.
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1, 101);
    println!("The random number is {}", random_number);
}
