// Write a program that will calculate the number of trailing zeros in a factorial of a given number.
//
// N! = 1 * 2 * 3 * ... * N
//
// Be careful 1000! has 2568 digits...
//
// For more info, see: http://mathworld.wolfram.com/Factorial.html
// Examples
//
// zeros(6) = 1
// # 6! = 1 * 2 * 3 * 4 * 5 * 6 = 720 --> 1 trailing zero
//
// zeros(12) = 2
// # 12! = 479001600 --> 2 trailing zeros
//
// Hint: You're not meant to calculate the factorial. Find another way to find the number of zeros.

fn zeros(n: u64) -> u64 {
    let mut zeros = 0;
    let mut value = 1;
    while value < n {
        value *= 5;
        zeros += n / value;
    }
    zeros
}

#[test]
fn sample_tests() {
    assert_eq!(zeros(0), 0);
    assert_eq!(zeros(6), 1);
    assert_eq!(zeros(14), 2);
    assert_eq!(zeros(30), 7);
    assert_eq!(zeros(1000), 249);
    assert_eq!(zeros(100000), 24999);
    assert_eq!(zeros(1000000000), 249999998);
}

fn main() {
    println!("Run: cargo test");
}
