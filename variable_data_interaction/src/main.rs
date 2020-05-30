fn main() {
    let _x = 5;
    let _y = _x;
    // println!("{}, {}", _x, _y );

    // let _s1 = String::from("hello");
    // let _s2 = _s1;
    // println!("{:?}", _s2);
     
    let _s1 = String::from("hello");
    let _s2 = _s1.clone();
    println!("_s1 = {}, _s2 = {}", _s1, _s2);
}
