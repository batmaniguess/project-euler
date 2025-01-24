const NUMBER: &str = "
73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450";

pub fn mutliple_three_five(bounds: u32, a: u32, b: u32) -> Result<u32, &'static str> {
    if bounds == 0 || a == 0 || b == 0 {
        return Err("Bounds and divisor must be non-zero");
    }
    let total = (0..bounds).filter(|&i| i % a == 0 || i % b == 0).sum();

    Ok(total)
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

pub fn largest_prime_factor(mut n: u64) -> Option<u64> {
    if n < 2 {
        return None;
    }
    let mut factor = 2;
    let mut largest = 0;

    while n > 1 {
        if n % factor == 0 {
            largest = factor;
            n /= factor;
            while n % factor == 0 {
                n /= factor;
            }
        }
        factor += 1;

        if factor * factor > n {
            if n > 1 {
                largest = n;
            }
            break;
        }
    }
    Some(largest)
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

pub fn find_primes(n: usize) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut count = 0;
    let mut candidate = 1;
    while count < n {
        candidate += 1;
        if is_prime(candidate) {
            count += 1;
        }
    }
    Some(candidate)
}

pub fn largest_product_series(digits: usize) -> Option<u64> {
    let number: Vec<u64> = NUMBER
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    if digits > number.len() {
        return None;
    }
    number
        .windows(digits)
        .map(|window| window.iter().product())
        .max()
}

pub fn special_pythagorean_triplet() -> Option<u64> {
    for a in 1..1000 {
        for b in (a + 1)..1000 {
            let c = 1000 - a - b;
            if c > b && a * a + b * b == c * c {
                return Some(a as u64 * b as u64 * c as u64);
            }
        }
    }
    None
}

pub fn summation_of_primes(num: u64) -> u64 {
    let mut sieve = vec![true; num as usize];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..=(num as f64).sqrt() as u64 {
        if sieve[i as usize] {
            for j in (i * i..num).step_by(i as usize) {
                sieve[j as usize] = false;
            }
        }
    }

    sieve
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(i, _)| i as u64)
        .sum()
}
