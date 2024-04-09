fn duplicate_encode(word: &str) -> String {
    let word_vec = word
        .split("")
        .map(|c| c.to_lowercase())
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>();
    let mut output = vec![];

    for letter in &word_vec {
        if word_vec.iter().filter(|&l| *l == *letter).count() == 1 {
            output.push("(");
        } else {
            output.push(")");
        };
    }

    return output.join("").to_string();
}

fn main() {
    let test = duplicate_encode("success");
    println!("{}", test);
}
