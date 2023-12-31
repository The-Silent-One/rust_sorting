use std::cmp::Ordering;
mod sorting;

pub fn fibonacci(n: u64) -> u64 {
    match n {
        1 | 2 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2)
    }
}

pub fn factorial(n: u128) -> u128 {
    match n {
        1 => 1,
        _ => n * factorial(n-1)
    }
}

fn main() {
    let n : u64 = 34;
    let m: u64 = 32;
    println!("factorial of {}: {}",n,factorial(n.into()));
    println!("Max of u128    : {}",u128::MAX);

    match n.cmp(&m) {
        Ordering::Less => println!("{} Less than {}",n,m),
        Ordering::Equal => println!("{} Equals to {}",n,m),
        Ordering::Greater => println!("{} Greater than {}",n,m),
    };
}
