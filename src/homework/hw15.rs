pub fn solve_muha_slon() {
    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let mut count = 0;

    for m in &digits {
        for u in &digits {
            for x in &digits {
                for a in &digits {
                    for s in &digits {
                        for l in &digits {
                            for o in &digits {
                                for n in &digits {
                                    let values = vec![m, u, x, a, s, l, o, n];
                                    let mut unique = values.clone();
                                    unique.sort();
                                    unique.dedup();
                                    if unique.len() != 8 {
                                        continue;
                                    }

                                    let muha = 1000 * m + 100 * u + 10 * x + a;
                                    let slon = 1000 * s + 100 * l + 10 * o + n;

                                    if muha * a == slon {
                                        count += 1;
                                        println!("{muha} × {a} = {slon}");
                                        println!("muxa = {muha}, a = {a}, slon = {slon}\n");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("комлексних рішень: {count}");
}
