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
