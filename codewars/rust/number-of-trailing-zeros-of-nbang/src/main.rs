#[allow(dead_code)]
fn check_fact(n: u64) -> u64 {
    let mut fact = 1;

    if n == 1 {
        return 1;
    }

    for x in 2..=n {
        fact *= x;
    }
    return fact;
}

fn zeros(n: u64) -> u64 {
    return std::iter::successors(Some(5u64), |&x| x.checked_mul(5))
        .take_while(|&x| x <= n)
        .map(|x| n / x)
        .sum();
}

fn main() {
    println!("{}", zeros(0));
    println!("{}", zeros(6));
    println!("{}", zeros(14));
    println!("{}", zeros(30));
}
