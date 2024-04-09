// # 27. Remove Element
//
// | https://leetcode.com/problems/remove-element/
//
// Given an array of integers `nums` and a integer `val`, remove all occurrences of `val` from
// `nums` **in-place** (mutate the array). The order of elements can be changed.
//
// ## Solution requirements
//
// Consider the number of non-`val` element in `nums` to be `k`.
// - Change the array `nums` such that the first `k` elements of `nums` contain the non-`val` elements;
//      remaining details of `nums` (array size, elements after `k`) are not important.
// - Return `k`.

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // println!("{:?} (remove '{}')", nums, val);

    nums.sort();
    num

    println!("{:?}", nums);

    return 0;
}

fn main() {
    let mut test_one: Vec<i32> = vec![3,2,2,3];
    let test_one_val: i32 = 3;

    remove_element(&mut test_one, test_one_val);
    println!("results: {:?} (remove all {})\n\n", test_one, test_one_val);

    // --------------------------------------------------------------------- //

    let mut test_two: Vec<i32> = vec![0,1,2,2,3,0,4,2];
    let test_two_val: i32 = 2;

    remove_element(&mut test_two, test_two_val);
    println!("results: {:?} (remove all {})\n\n", test_two, test_two_val);

}
