pub fn nth(n: u32) -> u32 {
    let mut prime = 2;
    for _ in 0..n {
        prime = next_prime(prime);
    }
    prime
}

pub fn next_prime(mut x: u32) -> u32 {
    while !is_prime(x+1) { x += 1; }
    x+1
}

pub fn is_prime(x: u32) -> bool {
    if x == 0 || x == 1 { return false; }
    for i in 2..(x/2)+1 { if x % i == 0 { return false; } }
    true
}
