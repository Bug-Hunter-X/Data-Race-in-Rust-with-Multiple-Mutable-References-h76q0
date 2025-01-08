fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is also a mutable reference to x

    *y = 10; // Modifying x through y
    *z = 15; // This will compile but leads to data races at runtime
}