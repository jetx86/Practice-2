fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

fn main() {
    let nums = [123, 121, 1221, 111, 10];
    for &x in &nums {
        println!("{x} => {}", is_palindrome(x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        data
            .iter()
            .for_each(|(n, exp)| {
                assert_eq!(is_palindrome(*n), *exp);
            });
    }
}
