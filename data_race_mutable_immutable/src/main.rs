fn main() {
    let mut _s = String::from("Hello");
    let _b = &_s;
    let _c = &_s;
    let _d = &_s;
    // cannot borrow '_s' as mutable, as it is also borrowed as imutable
    // you cannot have immutable pointer and mutable pointer in the same scope
    // let _a = &mut _s;
    // _a.push_str(" World!");
    // println!("{}, {}, {}, {}", _a, _b, _c, _d);
    
    println!("{}, {}, {}", _b, _c, _d);

    // The error will not occur here because
    // none of the immutable pointers are being used
    // after this
    let _a = &mut _s;
    _a.push_str(" World!");
    println!("{}", _a);
    // If you uncomment this the error will appear
    // println!("{}", _b);
}
