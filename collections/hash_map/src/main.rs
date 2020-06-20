use std::collections::HashMap;

fn main() {
    // Storing Keys with Associated Values in Hash Maps

    // Creating a New Hash Map

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    // Another way of constructing a hash map is by using
    // iterators and the collect method on a vector of tuples,
    // where each tuple consists of a key and its value
    // The `collect` method gathers data into a number of
    // collection types, including HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Following statement uses iter() instead of into_iter()
    // let scores: HashMap<_, _> =
    //     teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);

    // Hash Maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // Value borrowed here, after move
    // Because String is stored in Heap, on which Ownership rules are applied
    // println!("{:?} {:?}",field_name, field_value );

    println!("{:?}", map);


    let field_name = 50;
    let field_value = 1000;

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are valid at this point
    // because both belong to a basic data type which is stored
    // in Stack so Ownership rules do not apply to it
    println!("{:?} {:?}",field_name, field_value );

    println!("{:?}", map);

    // Accessing Values in a Hash Map

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // `get` returns Option enum, which is Some or None
    let score = scores.get(&team_name);

    println!("{:?}", score);

    // Accessing Values in a Hash Map through Loop
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // you can `&` pre-fix from scores and the code will still work
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a Hash Map

    // Overwriting a Value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);
    
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // It’s common to check whether a particular key has a value
    // and, if it doesn’t, insert a value for it.
    // Hash maps have a special API for this called entry
    // that takes the key you want to check as a parameter.
    // The return value of the entry method is an enum called `Entry`
    // that represents a value that might or might not exist.

    // The or_insert method on Entry is defined to return a
    // mutable reference to the value for the corresponding
    // Entry key if that key exists, and if not,
    // inserts the parameter as the new value for this key
    // and returns a mutable reference to the new value.

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value

    // Another common use case for hash maps is to look up a
    // key’s value and then update it based on the old value.
    
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
    	// `count` in this case is mutable reference
    	// to the value for the corresponding Entry key
        let count = map.entry(word).or_insert(0);
        // Here we store that mutable reference in the count variable,
        // so in order to assign to that value,
        // we must first dereference count using the asterisk (*)
        // The mutable reference goes out of scope at the end
        // of the for loop, so all of these changes are safe
        // and allowed by the borrowing rules.
        *count += 1;
    }

    println!("{:?}", map);

}
