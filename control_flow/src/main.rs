fn main() {
    
    let number = 0;

    if number < 0 {
    	println!("The number is negative");
    }
    
    else if number == 0 {
    	println!("The number is zero");
    }

    else {
    	println!("The number is positive");
    }

    // if let syntax

    let even = false;

    // the date type for number should return same type of value
    let number = {
    	if even {
    		6	//expression
    	} else {
    		7	// expression
    		// 'A'	// will throw an error expected integer found character
    	}
    };	//statment

    println!("{}", number);
}
