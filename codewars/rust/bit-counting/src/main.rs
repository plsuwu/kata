fn count_bits(n: i64) -> u32 {
    let mut bits: Vec<i64> = vec![];
    let mut current_bit = n;

    while current_bit > 0 {
        bits.push(current_bit % 2);
        current_bit = current_bit / 2;
    }

    bits.clone()
        .into_iter()
        .filter(|e| *e == 1)
        .collect::<Vec<_>>()
        .len()
        .try_into().unwrap()
}

#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    // assert_eq!(count_bits(7), 3);
    // assert_eq!(count_bits(9), 2);
    // assert_eq!(count_bits(10), 2);
}

fn main() {
    let num_bits = count_bits(7);
    dbg!(num_bits);
}
