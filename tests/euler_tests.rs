use project_euler::*;

#[test]
fn test_multiple_three_five() {
    assert_eq!(mutliple_three_five(10, 3, 5).unwrap(), 23);
    assert_eq!(mutliple_three_five(1000, 3, 5).unwrap(), 233168);
    assert!(mutliple_three_five(0, 3, 5).is_err());
}

#[test]
fn test_even_fibonacci() {
    assert_eq!(even_fibonacci(10), 10);
    assert_eq!(even_fibonacci(4_000_000), 4_613_732);
}

#[test]
fn test_largest_prime_factor() {
    assert_eq!(largest_prime_factor(13195).unwrap(), 29);
    assert_eq!(largest_prime_factor(600_851_475_143).unwrap(), 6857);
    assert_eq!(largest_prime_factor(1).unwrap_or(0), 0);
}

#[test]
fn test_sum_square_difference() {
    assert_eq!(sum_square_difference(10), 2640);
    assert_eq!(sum_square_difference(100), 25_164_150);
}

#[test]
fn test_find_primes() {
    assert_eq!(find_primes(6).unwrap(), 13);
    assert_eq!(find_primes(10001).unwrap(), 104_743);
}

#[test]
fn test_larget_product() {
    assert_eq!(largest_product_series(4).unwrap(), 5832);
    assert_eq!(largest_product_series(13).unwrap(), 23_514_624_000);
}
