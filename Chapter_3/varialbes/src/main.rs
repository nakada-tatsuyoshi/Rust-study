fn main() {
    let x = 1+3*2;
    println!("{x}");
    let t = true;
    let f: bool = false;
    println!("{t} {f}");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{0}", tup.1)
}
