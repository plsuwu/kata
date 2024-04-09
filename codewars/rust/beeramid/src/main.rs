#[cfg(test)]
mod tests {
    use super::beeramid;

    #[test]
    fn basic_tests() {
        assert_eq!(beeramid(9, 2.0), 1);
        assert_eq!(beeramid(10, 2.0), 2);
        assert_eq!(beeramid(11, 2.0), 2);
        assert_eq!(beeramid(21, 1.5), 3);
        assert_eq!(beeramid(454, 5.0), 5);
        assert_eq!(beeramid(455, 5.0), 6);
        assert_eq!(beeramid(4, 4.0), 1);
        assert_eq!(beeramid(3, 4.0), 0);
        assert_eq!(beeramid(0, 4.0), 0);
        assert_eq!(beeramid(-1, 4.0), 0);
    }
}

/// # Beeramid
/// | https://www.codewars.com/kata/51e04f6b544cf3f6550000c1/train/rust
///
/// # Problem
///
/// A beer can pyramid will square the number of cans in each level:
///     - 1 can in the top level,
///     - 4 in the second,
///     - 9 in the next,
///     - 16, 25...
/// Complete the beeramid function to return the number of complete levels of a beer can pyramid you can make, given the parameters of:
///     - your referral bonus,
///     - the price of a beer can.
///
/// # Examples
/// ```
/// beeramid(1500, 2); // should return `12`
/// beeramid(5000, 3); // should return `16`
/// ```
///
fn beeramid(bonus: i32, price: f32) -> usize {
    if (bonus as f32) < price {
        return 0;
    }

    if (bonus as f32) == price {
        return 1;
    }

    let mut layer_accumulator = 0;
    let mut layer_price = (layer_accumulator as f32).powf(2.0) * price;
    let mut money = bonus as f32 - layer_price;

    loop {
        layer_accumulator += 1;
        layer_price = (layer_accumulator as f32).powf(2.0) * price;

        if layer_price > money {
            layer_accumulator -= 1;
            break;
        }

        money -= layer_price;
    }

    return layer_accumulator;
}

fn main() {
    todo!();
}
