fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return None;
    }

    let average = total / n;
    let mut moves = 0;
    let mut balance = 0;

    for &load in shipments.iter() {
        balance += load as i32 - average as i32;
        moves += balance.abs() as usize;
    }

    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut base = 10;
    let mut v = Vec::new();

    for i in 0..n {
        v.push((base + (i % 3)) as u32);
    }

    let total: u32 = v.iter().sum();
    let remainder = total as usize % n;

    if remainder != 0 {
        v[0] -= remainder as u32;
    }

    v
}

fn main() {
    let example1 = vec![8, 2, 2, 4, 4];
    let example2 = vec![9, 3, 7, 2, 9];
    let example3 = vec![3, 5, 8];

    for example in [example1, example2, example3] {
        println!("Shipments: {:?}", example);
        match count_permutation(&example) {
            Some(steps) => println!("необхідні рухи: {}\n", steps),
            None => println!("не вдається збалансувати shipments.\n"),
        }
    }

    let generated = gen_shipments(5);
    println!("згенеровано valid shipments: {:?}", generated);
    if let Some(steps) = count_permutation(&generated) {
        println!("valid - можна збалансувати за {} ходів", steps);
    }
}
