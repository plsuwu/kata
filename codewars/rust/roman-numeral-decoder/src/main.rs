fn roman_as_num(roman: &str) -> u64 {
    let split_roman = roman
        .split("")
        .filter(|&c| !c.is_empty())
        .collect::<Vec<&str>>()
        .to_vec();

    let mut total = 0;

    println!("{:?}", split_roman);

    for (i, n) in split_roman.iter().enumerate() {
        println!("{} -> {}", i, n);
        if i < roman.len() - 1 {
            let positional = format!("{}{}", n, split_roman[i + 1]);
            match positional.as_str() {
                "IV" => total += 4,
                "IX" => total += 9,
                "XL" => total += 40,
                "XC" => total += 90,
                "CD" => total += 400,
                "CM" => total += 900,
                _ => match *n {
                    "I" => total += 1,
                    "V" => total += 5,
                    "X" => total += 10,
                    "L" => total += 50,
                    "C" => total += 100,
                    "D" => total += 500,
                    "M" => total += 1000,
                    _ => panic!("wuh woh"),
                },
            }
        } else {
            // need check for if roman[i-1] + roman[i] == "..."!!!
            match *n {
                "I" => total += 1,
                "V" => total += 5,
                "X" => total += 10,
                "L" => total += 50,
                "C" => total += 100,
                "D" => total += 500,
                "M" => total += 1000,
                _ => panic!("wuh woh"),
            }
        }
    }

    return total;
}

fn main() {
    println!("{}", roman_as_num("MXC"));
    println!("{}", roman_as_num("C"));
    println!("{}", roman_as_num("I"));
}
