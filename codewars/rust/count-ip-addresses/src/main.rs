use std::collections::HashMap;

fn ips_between(start: &str, end: &str) -> u32 {
    // SUBNET               No. POSSIBLE ADDRESSES
    // ----------------- | ----------------------- |
    // 255.255.255.255  ->  1
    // 255.255.255.0    ->  256
    // 255.255.0.0      ->  65,536
    // 255.0.0.0        ->  16,777,216
    // 0.0.0.0          ->  4,294,967,294

    // 0 -> 255 => (256 * (positions with a 0))

    // let mut net_map: HashMap<_, _> = HashMap::new();

    let start_octets: Vec<usize> = start.split('.').map(|o| o.parse().unwrap()).collect();
    let end_octets: Vec<usize> = end.split('.').map(|o| o.parse().unwrap()).collect();

    // (20.0.0.10, 20.0.1.0) => 246

    for (i, (s, e)) in start_octets
        .iter()
        .rev()
        .zip(end_octets.iter().rev())
        .enumerate()
    {
        println!("{}: {}, {}", i, s, e);
        if s - e != 0 {
            println!("{} - {} != 0.", s, e);
            println!("{}", (e - s) + (256 * (i + 1)));
        }
    }

    // println!("{:#?}, {:#?}", start_octets, end_octets);

    // let start_vec = start
    //     .split(".")
    //     .filter(|n| !n.is_empty())
    //     .map(|n| n.parse::<u32>().unwrap())
    //     .collect::<Vec<_>>();
    // let end_vec = end
    //     .split(".")
    //     .filter(|n| !n.is_empty())
    //     .map(|n| n.parse::<u32>().unwrap())
    //     .collect::<Vec<_>>();
    // println!("{:#?}", net_map);
    // println!("{:?}\n{:?}", start_vec, end_vec);

    // net_map.extend(start_vec.map(|s|));

    return 0;
}

fn main() {
    ips_between("20.0.0.10", "20.0.1.0");
}
