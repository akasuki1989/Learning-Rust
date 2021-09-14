use std::io;

fn main() {
    let mut temp = String::new();

    println!("Please input your number!");

    io::stdin()
        .read_line(&mut temp)
        .expect("Read variable is failed");
}