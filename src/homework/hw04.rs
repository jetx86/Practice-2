const SIZE: usize = 5; 
const MAX_WIDTH: usize = SIZE * 2 - 1; 

fn main() {
    for i in 0..SIZE {

        let stars = 2 * i + 1;
        let spaces = (MAX_WIDTH - stars) / 2;
        print_line(spaces, stars);
    }

    for i in (0..SIZE - 1).rev() {
        let stars = 2 * i + 1;
        let spaces = (MAX_WIDTH - stars) / 2;
        print_line(spaces, stars);
    }
}

fn print_line(spaces: usize, stars: usize) {
 
    for _ in 0..spaces {
        print!(" ");
    }
    for _ in 0..stars {
        print!("*"); 
    }
    println!(); 
}
