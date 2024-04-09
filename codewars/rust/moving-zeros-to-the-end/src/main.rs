pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(arr.len());
    let mut counter = 0;

    for &n in arr {
        if n == 0 {
            counter += 1
        } else {
            res.push(n);
        }
    }

    res.extend(vec![0; counter]);
    return res;
}

// fn move_zeros(xs: &[u8]) -> Vec<u8> {
//     let mut ys = Vec::with_capacity(xs.len());
//     ys.extend(xs.iter().filter(|&&x| x != 0));
//     ys.resize(xs.len(), 0);
//     ys
// }

fn main() {
    let testcase = move_zeros(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1]);

    println!("{:?}", testcase)
}
