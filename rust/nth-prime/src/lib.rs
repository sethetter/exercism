struct Prime {
    val: u32,
}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.val < 2 {
            self.val = 2;
            return Some(2);
        }

        let n = self.val + 1;
        for i in n..n*2 {
            if is_prime(i) {
                self.val = i;
                return Some(i);
            }
        }
        None
    }
}

pub fn nth(n: u32) -> u32 {
    let mut p = Prime{val: 0};
    match p.nth(n as usize) {
        None => 0,
        Some(x) => x,
    }
}

pub fn is_prime(x: u32) -> bool {
    if x == 0 || x == 1 { return false; }
    for i in 2..x-1 {
        if x != i && x % i == 0 { return false; }
    }
    true
}
