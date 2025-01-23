/*
* Euler Project
* */

fn mutliple_three_five(bounds: u32, a: u32, b: u32) -> u32 {
    let mut total: u32 = 0;
    for i in 0..bounds {
        if i % a == 0 || i % b == 0 {
            total += i;
        }
    }
    return total;
}

fn even_fibonacci(bounds: u32) -> u32 {
    let mut total: u32 = 0;
    let mut a: u32 = 1;
    let mut b: u32 = 2;

    while a <= bounds {
        if a % 2 == 0 {
            total += a;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    return total;
}

fn main() {
    println!("{}", mutliple_three_five(1000, 3, 5));
    println!("{}", even_fibonacci(4000000));
}
