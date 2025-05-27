use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    let mut min_sum = i32::MAX;
    let mut index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            index = i;
        }
    }

    (index, min_sum)
}

fn print_result(data: &[i32]) {
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:   [");
    for (i, v) in data.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{}", v);
    }
    println!("]");

    let (idx, min_sum) = min_adjacent_sum(data);

    print!("indexes:");
    for i in 0..data.len() {
        if i == idx {
            print!("\\__");
        } else if i == idx + 1 {
            print!(" __/");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[idx],
        data[idx + 1],
        min_sum,
        idx,
        idx + 1
    );
}

fn main() {
    let vec = gen_random_vector(20);
    print_result(&vec);
}
