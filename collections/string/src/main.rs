fn main() {

    // Creating a String
    let _s = String::new();

    println!("{}", _s);

    let _s = String::from("Initial Contents");

    println!("{}", _s);

    // Converting from str to String
    let _s = "Hello World!";

    let data = _s.to_string();

    println!("{}", data);

    let data = "Assalam-u-Alaikum".to_string();
    println!("{}", data);

    // Storing UTF-8 Encoded Text with String

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    println!("{}", _hello);

    // Appending to a String with push_str and push

    // using the `push_str` method to append a string slice
    let mut _s = String::from("foo");
    _s.push_str("bar");
    println!("{}", _s);

    // The `push` method takes a single character as a parameter and adds it to the String
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // With all of the `+` and `"` characters,
    // it's difficult to see what's going on
 
    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s);

    // For more complicated string combining,
    // we can use the `format!` macro
    
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // Indexing into Strings
    // Following code will throw an error,
    // In rust you cannot access index of a String
    
    // let s1 = String::from("hello");
    // let h = s1[0];
    
    let length = String::from("Hola").len();
    // The length of this String is 4 bytes long
    println!("{}", length);
    
    let len = String::from("Здравствуйте").len();
    // You must be thinking the length of the String is 12
    // But that's not the case, the length is 24 bytes long
    println!("{}", len);

    // Slicing Strings

    // UTF-8 String
    let hello = "Здравствуйте";

    // panicked at 'byte index 1 is not a char boundary; it is inside 'З'
    // let s = &hello[0..1];
    // println!("{}", s);
    
    let s = &hello[0..2];
    println!("{}", s);

    // English Alphabet String
    let hello = "hola";

    // Following code will work for english alphabets
    let s = &hello[0..1];
    println!("{}", s);

    let s = &hello[0..2];
    println!("{}", s);

    // if you need to perform operations on individual Unicode scalar values,
    // the best way to do so is to use the chars method. Calling chars on "السلام عليكم"
    // separates out and returns six values of type char,
    // and you can iterate over the result to access each element
    
    for c in "السلام عليكم".chars() {
        println!("{}", c);
    }

    for b in "السلام عليكم".bytes() {
        println!("{}", b);
    }
}
