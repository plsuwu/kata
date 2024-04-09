/// # 1544 - Make The String Great
/// | https://leetcode.com/problems/make-the-string-great/
/// ## ?
/// given a string, return that string modified such that it meets the requirements for a good string.
///
/// ### A 'good' string has the following properties:
/// - does not contain **two adjacent characters** `s[i]` && `s[i +1]`
///     where `s[i]` = an ascii char
///         && `s[i + 1]` = same ascii char of opposing case
///
/// ## examples
/// ```
/// // input 1
///  s = "leEeetcode" // expects "leetcode"
/// // either you choose i = 1 or i = 2, both will result "leEeetcode" to be reduced to "leetcode".
///
/// // input 2
/// s = "abBAcC" // expects ""
/// // many possible scenarios, and all lead to the same answer.
/// // e.g "abBAcC" --> "aAcC" --> "cC" --> ""
/// //      "abBAcC" --> "abBA" --> "aA" --> ""
/// ```
fn make_good(s: String) -> String {
    let mut result: Vec<char> = Vec::new();

    // 'c' == 'C'
    //      ? pop('a').then(skip rest of current iteration)
    //      : append 'c' at the end of the result array.
    //
    // retains ordering of array operations but only iterates once
    for c in s.chars() {
        if let Some(&last) = result.last() {
            if last != c && last.eq_ignore_ascii_case(&c) {
                result.pop();
                continue;
            }
        }

        result.push(c);
    }

    result.into_iter().collect()
}

fn main() {

    // test cases but printable
    let cases: Vec<_> = Vec::from([["abcd", "abcd"], ["leEeetcode", "leetcode"], ["abBAcC", ""]]);

    let cases_ret = cases
        .iter()
        .map(|c| {
            let string = c[0].to_string();
            return make_good(string);
        })
        .collect::<Vec<_>>();

    for (i, c) in cases.iter().enumerate() {
        println!(
            "case={}: input '{}' returns '{}' (expects '{}').",
            i,
            c[0],
            cases_ret[i].to_string(),
            c[1]
        );
    }
}
