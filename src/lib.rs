pub fn mutliple_three_five(bounds: u32, a: u32, b: u32) -> u32 {
    let mut total = 0;
    for i in 0..bounds {
        if i % a == 0 || i % b == 0 {
            total += i;
        }
    }
    total
}

pub fn even_fibonacci(bounds: u32) -> u32 {
    let mut total = 0;
    let mut a = 1;
    let mut b = 2;

    while a <= bounds {
        if a % 2 == 0 {
            total += a;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    total
}

pub fn largest_prime_factor(mut n: u64) -> u64 {
    let mut factor = 2;
    let mut largest = 0;

    while n > 1 {
        while n % factor == 0 {
            largest = factor;
            n /= factor;
        }
        factor += 1;

        if factor * factor > n && n > 1 {
            largest = n;
            break;
        }
    }
    largest
}

pub fn sum_square_difference(n: u32) -> u32 {
    let sum_of_squares = n * (n + 1) * (2 * n + 1) / 6;
    let square_of_sums = (n * (n + 1) / 2) * (n * (n + 1) / 2);
    square_of_sums - sum_of_squares
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn find_primes(n: usize) -> u64 {
    let mut count = 0;
    let mut candidate = 1;
    while count < n {
        candidate += 1;
        if is_prime(candidate) {
            count += 1;
        }
    }
    candidate
}
