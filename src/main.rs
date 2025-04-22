mod karacas;

fn main() {
    let mut h = String::from("Hello World!");
    println!("{h}");
    karacas::encrypt(&mut h);
    println!("{h}");
}