fn parse(code: &str) -> Vec<i32> {
    // i -> incr val
    // d -> decr val
    // s -> square val
    // o -> output val into return vec

    let mut val: i32 = 0;
    let mut out: Vec<i32> = vec![];

    let _ = code.split("").for_each(|c| {
        match c {
            "i" => val += 1,
            "d" => val -= 1,
            "s" => val = val.pow(2),
            "o" => out.push(val),
            _ => (),
        }

        println!("{}", &val);
    });

    out
}

fn main() {
    let test_case = parse("iiisdoso");
    println!("{:?}", test_case);
}
