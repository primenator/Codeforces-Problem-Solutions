use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let int_val: i32 = parts[0].parse().unwrap();
    let long_val: i64 = parts[1].parse().unwrap();
    let char_val: char = parts[2].chars().next().unwrap();
    let float_val: f32 = parts[3].parse().unwrap();
    let double_val: f64 = parts[4].parse().unwrap();

    println!("{}", int_val);
    println!("{}", long_val);
    println!("{}", char_val);
    println!("{}", float_val);
    println!("{:.1}", double_val);

}

