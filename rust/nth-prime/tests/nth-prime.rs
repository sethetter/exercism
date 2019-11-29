use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}

// @sethetter: my tests

#[test]
fn test_is_prime() {
    let primes: &[u32] = &[2,3,5,7,13,17,23];
    for x in primes {
        assert_eq!(np::is_prime(*x), true);
    }

    let not_primes: &[u32] = &[0,1,4,6,8,9];
    for x in not_primes {
        assert_eq!(np::is_prime(*x), false);
    }
}
