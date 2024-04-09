/// # Common Denominators
/// | https://www.codewars.com/kata/54d7660d2daf68c619000d95/train/rust
///
/// ## Description
///
/// Given a vector of positive rational numbers in the form
/// ```
/// [ (numer_1, denom_1) , ... (numer_n, denom_n) ]
/// ```
/// Return a result in the form
/// ```
/// [ (N_1, D) , ... (N_n, D) ]
///
/// // ie,
/// ([1, 2], [1, 3], [1, 4])
///
/// // should return
/// [(6, 12), (4,12), (3,12)]
///
/// // and ...
/// [(69, 130), (87, 1310), (3, 4)]
///
/// // should return
/// [(18078, 34060), (2262, 34060), (25545, 34060)]
///
/// ```
///
fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let (numerators, denominators): (Vec<_>, Vec<_>) = l.iter().cloned().unzip();
    println!("{:?} / {:?}", numerators, denominators);

    // maybe i divide thgem idk
    for (i, n) in numerators.iter().enumerate() {
        println!("{} / {}", n, denominators[i]);
        let output: f32 = *n as f32 / denominators[i] as f32;
        let testoutput: f32 = 6 as f32 / 12 as f32;
        println!("{}, {}", output, testoutput);
    }

    return l;

}

fn main() {
    // let test = vec![(1,2),(1,3),(1,4)];
    let test = vec![(69, 130), (87, 1310), (3, 4)];
    // let test_expectation = [(6,12), (4,12), (3,12)];
    let test_expectation = [(18078, 34060), (2262, 34060), (25545, 34060)];
    let res = convert_fracts(test.clone());
    println!("\n---\nresults\n---\n{:?} \n   =wants-> {:?}, \n    and got {:?}", test, test_expectation, res);
}
