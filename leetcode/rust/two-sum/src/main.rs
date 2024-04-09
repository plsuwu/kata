pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut viable: Vec<i32> =
        if target - &nums.iter().copied().min().unwrap() <= target && target > 0 {
            nums.iter().copied().filter(|n| n <= &target).collect()
        } else {
            nums.clone()
        };

    if viable.len() == 2 && viable.iter().fold(0, |acc, x| acc + x) == target {
        nums.iter()
            .enumerate()
            .filter(|(_, n)| **n == viable[0] || **n == viable[1])
            .map(|(i, _)| i.try_into().unwrap())
            .for_each(|n| result.push(n));
    } else {

        // binary search
        viable.sort();

        let mut hi = viable.len() - 1;
        let mut lo = 0;

        while lo < hi {
            let b_search = viable[lo] + viable[hi];

            if b_search > target {
                hi = hi - 1;
            }

            if b_search < target {
                lo = lo + 1;
            }

            if b_search == target && lo != hi {
                break;
            }
        }

        // push output from binary search to result vec
        nums.iter()
            .enumerate()
            .filter(|(_, n)| **n == viable[lo] || **n == viable[hi])
            .map(|(i, _)| i.try_into().unwrap())
            .for_each(|n| result.push(n));
    }

    return result;
}

fn main() {
    println!(
        "returned from 2sum: {:?}\n----------\n************\n",
        two_sum(vec![0, 3, -3, 4, -1], -1)
    );
    println!(
        "returned from 2sum: {:?}\n----------\n************\n",
        two_sum(vec![3, 2, 3], 6)
    );
    println!(
        "returned from 2sum: {:?}\n----------\n************\n",
        two_sum(vec![-10, -1, -18, -19], -19)
    );
    println!(
        "returned from 2sum: {:?}\n----------\n************\n",
        two_sum(vec![6, 5, 7, 8, 9, 3], 10)
    );
}
