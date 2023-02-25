fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value og y is: {}", y);
    println!("five_hundred is: {}", tup.0);
    println!("siz_point_four is: {}", tup.1);
    println!("one is: {}", tup.2);
}
