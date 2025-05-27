const WIDTH: usize = 28;
const HEIGHT: usize = 10;

fn main() {
    let mut output = String::new();

    output.push('+');
    output.push_str(&"-".repeat(WIDTH - 2));
    output.push('+');
    output.push('\n');

    let inner_width = WIDTH - 4;
    let steps = HEIGHT - 2;

    for i in 0..steps {
        output.push('|');

        let left = i * inner_width / (2 * (steps - 1));
        let right = left;
        let middle = inner_width - left - right;

        output.push_str(&" ".repeat(left));
        output.push('\\');
        output.push_str(&" ".repeat(middle));
        output.push('/');
        output.push_str(&" ".repeat(right));
        output.push('|');
        output.push('\n');
    }

    output.push('+');
    output.push_str(&"-".repeat(WIDTH - 2));
    output.push('+');

    print!("{}", output);
}
