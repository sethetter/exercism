pub fn nth(n: u32) -> u32 {
    let mut i = 0;
    let mut prime = 2;
    while i < n {
        i += 1;
        prime += 1;
        while !is_prime(prime) {
            prime += 1;
        }
    }
    prime
}

pub fn is_prime(x: u32) -> bool {
    if x == 0 || x == 1 { return false; }
    for i in 2..x-1 { if x % i == 0 { return false; } }
    true
}
