const NUM_TRIANGLES: usize = 5;

fn main() {
    let mut tree_lines = Vec::new();

    for tri in 0..NUM_TRIANGLES {
        for row in 0..=tri {
            let stars = 1 + 2 * row;
            let max_width = 1 + 2 * (NUM_TRIANGLES - 1);
            let padding = max_width.saturating_sub(stars) / 2;

            let line = " ".repeat(padding) + &"*".repeat(stars);
            tree_lines.push(line);
        }
    }

    println!("{}", tree_lines.join("\n"));
}
