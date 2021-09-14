fn main() {
    let guess: u32 = "42".parse().expect("Not a number");

    println!("{}", guess);

    let t = true;

    println!("{}", t);

    let tup = (500, 46.2, "hi");

    let (_x, _y, _z) = tup;

    println!("{} {} {}", tup.0, tup.1, tup.2);

    let a = [3;5];
    
    println!("{}", a[1]);
}
