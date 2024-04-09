// fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
//     if k <= 0 || k > strarr.len() || strarr.len() == 0 {
//         return "".to_string();
//     }
//
//     let mut result: String = String::new();
//     let mut current_test: String = String::new();
//
//     strarr.iter().enumerate().for_each(|(i, _)| {
//         if i < strarr.len() - 1 {
//             if k > 1 {
//                 for w in strarr[i..i + k].iter() {
//                     current_test = format!("{}{}", current_test, w);
//                 }
//             } else {
//                 current_test = format!("{}", strarr[i + 1]);
//             }
//
//             if current_test.chars().count() > result.chars().count() {
//                 result = current_test.to_owned();
//             }
//             current_test = String::new();
//         }
//     });
//
//     return result;
// }

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if k <= 0 || k > strarr.len() {
        return "".to_string();
    }

    let mut res = String::new();
    let mut max_len = 0;

    for win in strarr.windows(k) {
        let cur_str = win.join("");
        let cur_len = cur_str.len();

        if cur_len > max_len {
            res = cur_str;
            max_len = cur_len;
        }
    }

    return res;
}

fn main() {
    let test = longest_consec(
        vec!["tree", "foling", "trashy", "blue", "abcdef", "uvwxyz"],
        2,
    );
    let test_two = longest_consec(
        vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ],
        1,
    );
    let test_three = longest_consec(
        vec![
            "llloosssulx",
            "bkkkiiigaaqq",
            "iifffaaad",
            "wwhaa",
            "izzziifpppk",
            "uuufffllll",
            "rrrskkkl",
            "tmfpmmlll",
            "ffftfffdnnn",
        ],
        8,
    );
    let test_four = longest_consec(vec![], 3);
    println!("test A -> {}", test);
    println!("test B -> {}", test_two);
    println!("test C -> {}", test_three);
    println!("test D -> {}", test_four);
}
