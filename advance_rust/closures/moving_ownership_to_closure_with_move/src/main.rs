fn main() {
    let x = vec![1, 2, 3];

    // If you want to force the closure to take
    // ownership of the values it uses in the
    // environment, you can use the `move`
    // keyword before the parameter list.
    let equal_to_x = move |z| z == x;

    // Will throw an error, x was borrowed
    // by the closure
    // comment below line to remove the error
    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
