use project_euler::*;

#[test]
fn test_multiple_three_five() {
    assert_eq!(mutliple_three_five(10, 3, 5), 23);
    assert_eq!(mutliple_three_five(1000, 3, 5), 233168);
}

#[test]
fn test_even_fibonacci() {
    assert_eq!(even_fibonacci(10), 10);
    assert_eq!(even_fibonacci(4000000), 4613732);
}

#[test]
fn test_largest_prime_factor() {
    assert_eq!(largest_prime_factor(13195), 29);
    assert_eq!(largest_prime_factor(600851475143), 6857);
}

#[test]
fn test_sum_square_difference() {
    assert_eq!(sum_square_difference(10), 2640);
    assert_eq!(sum_square_difference(100), 25164150);
}

#[test]
fn test_find_primes() {
    assert_eq!(find_primes(6), 13);
    assert_eq!(find_primes(10001), 104743);
}

#[test]
fn test_larget_product() {
    assert_eq!(largest_product_series(4), 5832);
    assert_eq!(largest_product_series(13), 23514624000);
}
