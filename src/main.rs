
fn main() {
    let mut a = 0;
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();

        if s.trim().is_empty() {
            break;
        }

        if let Some(first) = s.find(char::is_numeric) {
            let last = s.rfind(char::is_numeric).unwrap();

            let first = s.chars().nth(first).unwrap();
            let last = s.chars().nth(last).unwrap();

            a += format!("{}{}", first, last).parse::<i32>().unwrap();
        }
    };
    println!("{a}");
}
