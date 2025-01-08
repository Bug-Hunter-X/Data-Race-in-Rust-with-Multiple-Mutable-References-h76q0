fn main() {
    let mut x = 5;
    let mut y = x; // No mutable reference here
    let mut z = x;

    y = 10;
    z = 15;
    println!("y = {}, z = {}, x = {}",y, z, x);
}
