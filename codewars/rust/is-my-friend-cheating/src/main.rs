/// # Is My Friend Cheating?
/// | https://www.codewars.com/kata/5547cc7dcad755e480000004/train/rust
///
/// ## Description
/// - Using the sequence of all numbers from 1 to `n` (where `n` > 0) - `s`,
/// - `a` and `b` refer to two numbers within that sequence
/// - the product of `a` and `b` should be equal to the sum of all numbers in `s` excluding `a` and
///     `b`.
/// - Given `n`, return all possible combinations of `a` and `b`, or an empty array if no possible
///     numbers can be found.
/// ```
/// // ie,
/// remove_nb(n) = `[(a, b), (a, b), ...]`
/// // or,
/// remove_nb(n) = `[]` // impossible `a` and `b`.
///
fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    if m < 10 {
        return vec![];
    }

    let mut res = Vec::new();

    let ml = m as i64;
    let sum: i64 = (1..=ml).sum();

    for i in 1..ml {
        let n = sum - i;
        let d = i + 1;

        if n % d == 0 {
            let j = n / d;
            if j < ml && j != i {
                res.push((i as i32, j as i32));
            }
        }
    }

    return res;
}


// let m64 = m as i64;
// let sum: i64 = (1..=m64).sum::<i64>();
// let u_bound = (m64 as f64 * 0.1) as i64;
// let mut res: Vec<(i32, i32)> = Vec::new();
//
// for i in (m64 / 2) + 1..m64 - u_bound {
//     let max_j = sum / i;
//     for j in i + 1..max_j {
//         if i * j == sum - (i + j) {
//             res.push((i as i32, j as i32));
//             res.push((j as i32, i as i32));
//         } else if i * j > sum - (i + j) {
//             break;
//         }
//     }
// }

fn main() {
    let tb = remove_nb(1000);
    println!("(ta) 1000: {:?}\n", tb);

    let tb = remove_nb(i32::MAX);
    println!("tMAX : {:?}", tb);
    // let tc = remove_nb(17);
    // println!("(tb) 17: {:?}\n", tc);
    // let ta = remove_nb(26);
    // println!("(tc) 26: {:?}\n", ta);
    // let td = remove_nb(36);
    // println!("(td) 36: {:?}\n", td);

    let mut i = 1000;
    let mut res = Vec::new();
    loop {
        i += 1;
        let ab = remove_nb(i);

        if ab != vec![] {
            res.push(ab);
            break;
        }
    }
    println!("found first n ({}) with valid (a,b) -> {:?}", i, res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(remove_nb(1), vec![]);
        assert_eq!(remove_nb(1000), vec![]);
    }
    #[test]
    fn basic() {
        assert_eq!(remove_nb(26), vec![(15, 21), (21, 15)]);
        assert_eq!(remove_nb(1000), vec![]);
        assert_eq!(remove_nb(101), vec![(55, 91), (91, 55)]);
        assert_eq!(remove_nb(102), vec![(70, 73), (73, 70)]);
    }
}
