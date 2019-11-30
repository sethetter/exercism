pub fn nth(n: u32) -> u32 {
    match (2..).filter(|x| is_prime(*x)).nth(n as usize) {
        Some(x) => x,
        None => 0,
    }
}

pub fn is_prime(x: u32) -> bool {
    match x {
        0 | 1 => false,
        _ => match (2..).skip_while(|&y| x % y != 0 && x != y).nth(0) {
            None => false,
            Some(y) => y == x,
        }
    }
}
