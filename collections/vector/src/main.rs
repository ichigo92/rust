fn main() {
    let mut _v: Vec<i32> = Vec::new();

    // Creating Vector using vec macro
    let mut _v1 = vec![10, 20, 30, 40];

    println!("{:?}", _v);
    println!("{:?}", _v1);

    // to add value in a vector, use push method
    // push adds a value at the end of the vector
    _v.push(50);

    _v1.push(50);

    println!("{:?}", _v);
    println!("{:?}", _v1);

    // to remove a value from a vector, use pop method
    // pop removes the value in the last index (at the end of the vector)

    _v.pop();
    _v1.pop();

	println!("{:?}", _v);
    println!("{:?}", _v1); 

    // Vector drops after scope ends
    {
    	let v = vec![20, 30];
    	// v is available in this scope, so no error
    	println!("{:?}", v);
    }

    // will throw an error, cannot find value `v` in this scope
    // println!("{:?}", v);
    

    // Accessing value in vector
    let v = vec![2, 4, 6, 8, 10];

    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 6'
    // let _element1 = v[6];
    let _element1 = v[2];
    println!("{:?}", _element1);

    // Using `get` method to access a value, pass index to the method
    // If a value exists it will return `Some` otherwise `None`
    // It's a safe way to access a value in a Vector
    let _element2 = v.get(2);
    println!("{:?}", _element2);

    // to extract data from `Some` we will use binding operator
    match _element2 {
    	Some(value) => println!("{:?}", value),
    	None => println!("nothing")
    }

    // to create a reference of the value at a particular index
    // add `&` before the variable
    let _element3 = &v[2];

    // another way is, you can tell rust the data type of value
    // that will be returned
    let _element3: &i32 = &v[2];

    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 6'
    // let _element3 = &v[6];
    println!("{:?}", _element3);

    // note: to avoid in a vector
    // attempting to add an element to a vector while holding
    // a reference to an item
    /*let mut v = vec![50, 100, 150, 200];

    let element = &v[0];

    v.push(250);

    println!("{}", element);*/

    // Iterating over the Values in a Vector
    // the problem here is we have passed the
    // ownership of vector `v` to the for loop
    for i in v {
    	println!("{}", i);
    }

    // will throw an error as the ownership has moved to
    // the for loop, for loop has borrowed vector v
    // println!("{:?}", v);

    // to solve it we will use `mut` keyword

    let mut v = vec![10, 20, 30, 40, 50];

    for i in &mut v {

    	// i = i + 50
    	// because reference is passed here
    	// to change value of the variable
    	// we need to dereference
    	// in order to dereference we prefix the
    	// variable with `*` operator
    	*i += 50
    }

    println!("{:?}", v);


    // Using an Enum to Store Multiple Types

    // Think of an Excel Sheet Cell
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
