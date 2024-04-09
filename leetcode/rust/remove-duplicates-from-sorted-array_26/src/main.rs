/// # 26. Remove Duplicates from Sorted Array
///
/// | https://leetcode.com/problems/remove-duplicates-from-sorted-array
///
/// ## Descr
/// Given an array of integers sorted in non-decreasing order, `nums`, remove duplicate elements
/// **in-place** such that the array contains a single copy of each number.
/// Return the number of unique elements in `nums`.
///
/// ## A passing solution has the following functionality:
/// - change `nums` so that its first `k` elements contain the original order. size of `nums` and its remaining elements
///     are unimportant.
/// - `remove_duplicates` returns `k`.
///
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

fn main() {
    let mut test_one: Vec<i32> = vec![1, 1, 2];
    let test_one_immut: Vec<i32> = vec![1, 1, 2];

    let test_one_expected_mut: Vec<i32> = vec![1,2];
    let test_one_expected_ret: i32 = 2;

    println!("\n\ntest input -> {:?}", test_one_immut);
    println!("expected results -> \n    mutated nums array: {:?}, \n    return val: {}", test_one_expected_mut, test_one_expected_ret);
    remove_duplicates(&mut test_one);
    println!("processed array: {:?}", test_one);
}
