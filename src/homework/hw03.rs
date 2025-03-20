const WIDTH: usize = 10;
const HEIGHT: usize = 5;

fn main() {
    let mut output = String::new();

    output.push('+');
    output.push_str(&"-".repeat(WIDTH - 2));
    output.push('+');
    output.push('\n');

    for i in 0..HEIGHT - 2 {
        output.push('|');
        output.push_str(&" ".repeat(i));
        output.push('\\');
        output.push_str(&" ".repeat(WIDTH - 4 - 2 * i));
        output.push('/');
        output.push_str(&" ".repeat(i));
        output.push('|');
        output.push('\n');
    }

    output.push('+');
    output.push_str(&"-".repeat(WIDTH - 2));
    output.push('+');

    print!("{}", output);
}
