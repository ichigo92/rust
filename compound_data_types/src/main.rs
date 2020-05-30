fn main() {
    let _student: (u32, char, f64) = (25, 'A', 80.5);

    println!("{}", _student.0);	// 25
    println!("{}", _student.1);	// 'A'
    println!("{}", _student.2);	// 80.5

    println!("Destructure");

    // expected a tuple with 3 elements, found one with 2 elements
    // let (x, y) = _student;
    let (x, y, z) = _student;

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    println!("Arrays");

    				// type; length
    let _lottery_number: [u32; 5] = [1, 23, 45, 56, 78];
    println!("{:?}", _lottery_number);

    // if same numbers in an array
    let _rupee_five = [5; 10];		// 5 is the element, and length of array is 10
    println!("{:?}", _rupee_five);

    // character (char) array
    let days = ['M', 'T', 'W',];
    println!("{:?}", days[1]);

}
