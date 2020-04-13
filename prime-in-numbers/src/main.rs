// Given a positive number n > 1 find the prime factor decomposition of n. The result will be a string with the following form :
//
//  "(p1**n1)(p2**n2)...(pk**nk)"
//
// with the p(i) in increasing order and n(i) empty if n(i) is 1.
//
// Example: n = 86240 should return "(2**5)(5)(7**2)(11)"

fn prime_factors(n: i64) -> String {
    let mut res = String::new();
    let mut rest = n;
    // only need to test up to half of the given number
    for i in 2..(n/2) {
        // no need to test if (i) is prime
        // since 2 will take care of all its higher multiples (4, 6, 8, ...)
        if rest % i == 0 {
            res += &("(".to_owned() + &i.to_string());
            rest /= i;
            let mut count = 1;
            // if able to divide with (i), try again till fail
            while rest % i == 0 {
                rest /= i;
                count += 1;
            }
            if count > 1 {
                res += &("**".to_owned() + &count.to_string());
            }
            res += ")";
        }
    }
    // if nothing was found
    // we can assume that (n) is prime
    if res.chars().count() == 0 {
        res += &("(".to_owned() + &n.to_string() + &")".to_owned());
    }
    res
}

fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17*17*93*677, "(3)(17**2)(31)(677)");
    testing(7919, "(7919)");
}

fn main() {
    println!("Run: cargo test");
}
