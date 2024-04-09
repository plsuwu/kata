use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<char>>();
    let mut set = HashSet::new();
    let mut r_max = 0;
    let mut l = 0;

    for (r, &c) in chars.iter().enumerate() {
        while set.contains(&c) {
            set.remove(&chars[l]);
            l += 1;
        }

        set.insert(c);
        r_max = r_max.max(r - l + 1);
    }

    r_max as i32
}

fn main() {
    // let case_one = length_of_r_max_substring("abcabcbb".to_string());
    // let case_two = length_of_r_max_substring("bbbbb".to_string());
    let case_three = length_of_longest_substring("pwwkew".to_string());
    //
    // println!("1: '{}'\n2: '{}'\n3: '{}'", case_one, case_two, case_three);
}
