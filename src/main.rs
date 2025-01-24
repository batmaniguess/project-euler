/*
* Euler Project
* */

use project_euler::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", mutliple_three_five(1000, 3, 5)?);
    println!("{}", even_fibonacci(4000000));
    println!("{}", largest_prime_factor(600851475143).unwrap());
    println!("{}", sum_square_difference(100));
    println!("{}", find_primes(10001).unwrap());
    println!("{}", largest_product_series(13).unwrap());

    Ok(())
}
