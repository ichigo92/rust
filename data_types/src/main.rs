fn main() {
    // scalar Data Types
    let age = 25;				// int
    let percentage = 80.3;		// float
    let grade = 'A';			// char
    let passed = true;			// boolean

    let secret_number: i32 = -80;
    println!("{}", secret_number);

    // cannot apply uniary operator '-' to type `u32`
    let secret_number: u32 = -80;
    println!("{}", secret_number);

}
