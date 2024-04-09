// use preloaded::ALPHA;
//-


fn block_print(input: &str) {

    let char_vec: Vec<_> = input
        .split("")
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>();

    // let mapped = &ALPHA.iter().collect::<Vec<_>>();
    // println!("{:?}", mapped);


    println!("{:?}", char_vec)



    // for char in map_char {
    //     unimplemented!();
    // }
}

fn main() {
    let test = block_print("hellow worLD");
    // println!("{}", test)
}
