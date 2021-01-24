To create a binary project

cargo new <project name>


To create a library

cargo new --lib <project name>

To check files of a project

sudo apt-get install tree

tree .

### Package

Several rules determine what a package can contain. A package must contain zero or one library crates, and no more. It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).

main.rs
It is the crate root of binary

lib.rs is the crate root of library
`my_project` is refered to as a crate


#### Multiple Binary Crates
A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.


```bash
└── my_project
   ├── Cargo.toml
   └── src
        ├── bin
        │   ├── a.rs
        │   ├── b.rs
        │   └── c.rs
        └── main.rs
```

#### How crates help us?
1. Share functionalities
2. Easy debugging
3. Organized code
4. It makes our code Maintainable.
5. It makes our code Modular


### Modules
Modules let us organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

By using modules, we can group related definitions together and name why they’re related. Programmers using this code would have an easier time finding the definitions they wanted to use because they could navigate the code based on the groups rather than having to read through all the definitions. Programmers adding new functionality to this code would know where to place the code to keep the program organized.

Earlier, we mentioned that `src/main.rs` and `src/lib.rs` are called `crate roots`. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.

```bash
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

This tree shows how some of the modules nest inside one another (for example, hosting nests inside front_of_house). The tree also shows that some modules are *siblings* to each other, meaning they’re defined in the same module (hosting and serving are defined within front_of_house). To continue the family metaphor, if module A is contained inside module B, we say that module A is the *child* of module B and that module B is the *parent* of module A. Notice that the entire module tree is rooted under the implicit module named crate.

The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison! Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.

### Paths for Referring to an Item in the Module Tree
To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. If we want to call a function, we need to know its path.

A path can take two forms:

  -  An *absolute* path starts from a crate root by using a crate name or a literal crate.
  -  A *relative* path starts from the current module and uses self, super, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

Let’s return to the example in Listing 7-1. How do we call the `add_to_waitlist` function? This is the same as asking, what’s the path of the `add_to_waitlist` function? In Listing 7-3, we simplified our code a bit by removing some of the modules and functions. We’ll show two ways to call the `add_to_waitlist` function from a new function `eat_at_restaurant` defined in the crate root. The `eat_at_restaurant` function is part of our library crate’s public API, so we mark it with the `pub` keyword. In the ”Exposing Paths with the pub Keyword” section, we’ll go into more detail about pub. Note that this example won’t compile just yet; we’ll explain why in a bit.


Choosing whether to use a relative or absolute path is a decision you’ll make based on your project. The decision should depend on whether you’re more likely to move item definition code separately from or together with the code that uses the item. For example, if we move the `front_of_house` module and the `eat_at_restaurant` function into a module named `customer_experience`, we’d need to update the absolute path to `add_to_waitlist`, but the relative path would still be valid. However, if we moved the `eat_at_restaurant` function separately into a module named dining, the absolute path to the `add_to_waitlist` call would stay the same, but the relative path would need to be updated. Our preference is to specify absolute paths because it’s more likely to move code definitions and item calls independently of each other.

The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are private by default. Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. The reason is that child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined. To continue with the restaurant metaphor, think of the privacy rules as being like the back office of a restaurant: what goes on in there is private to restaurant customers, but office managers can see and do everything in the restaurant in which they operate.

Rust chose to have the module system function this way so that hiding inner implementation details is the default. That way, you know which parts of the inner code you can change without breaking outer code. But you can expose inner parts of child modules' code to outer ancestor modules by using the `pub` keyword to make an item public.


#### Starting Relative Paths with super
We can also construct relative paths that begin in the parent module by using super at the start of the path. This is like starting a filesystem path with the .. syntax. Why would we want to do this?

Consider the code in Listing 7-8 that models the situation in which a chef fixes an incorrect order and personally brings it out to the customer. The function fix_incorrect_order calls the function serve_order by specifying the path to serve_order starting with super:

The fix_incorrect_order function is in the back_of_house module, so we can use super to go to the parent module of back_of_house, which in this case is crate, the root. From there, we look for serve_order and find it. Success! We think the back_of_house module and the serve_order function are likely to stay in the same relationship to each other and get moved together should we decide to reorganize the crate’s module tree. Therefore, we used super so we’ll have fewer places to update code in the future if this code gets moved to a different module.

#### Making Structs and Enums Public
We can also use pub to designate structs and enums as public, but there are a few extra details. If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis. In Listing 7-9, we’ve defined a public back_of_house::Breakfast struct with a public toast field but a private seasonal_fruit field. This models the case in a restaurant where the customer can pick the type of bread that comes with a meal, but the chef decides which fruit accompanies the meal based on what’s in season and in stock. The available fruit changes quickly, so customers can’t choose the fruit or even see which fruit they’ll get.

### Bringing Paths into Scope with the `use` Keyword
It might seem like the paths we’ve written to call functions so far are inconveniently long and repetitive. For example, in Listing 7-7, whether we chose the absolute or relative path to the add_to_waitlist function, every time we wanted to call add_to_waitlist we had to specify front_of_house and hosting too. Fortunately, there’s a way to simplify this process. We can bring a path into a scope once and then call the items in that path as if they’re local items with the use keyword.

In Listing 7-11, we bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant function so we only have to specify hosting::add_to_waitlist to call the add_to_waitlist function in eat_at_restaurant.

Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.

You can also bring an item into scope with use and a relative path. Listing 7-12 shows how to specify a relative path to get the same behavior as in Listing 7-11.

#### Creating Idiomatic use Paths
In Listing 7-11, you might have wondered why we specified use crate::front_of_house::hosting and then called hosting::add_to_waitlist in eat_at_restaurant rather than specifying the use path all the way out to the add_to_waitlist function to achieve the same result, as in Listing 7-13.

Although both Listing 7-11 and 7-13 accomplish the same task, Listing 7-11 is the idiomatic way to bring a function into scope with use. Bringing the function’s parent module into scope with use so we have to specify the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path. The code in Listing 7-13 is unclear as to where add_to_waitlist is defined.

On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way to bring the standard library’s HashMap struct into the scope of a binary crate.

There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that. Listing 7-15 shows how to bring two Result types into scope that have the same name but different parent modules and how to refer to them.


#### Providing New Names with the `as` Keyword
There’s another solution to the problem of bringing two types of the same name into the same scope with use: after the path, we can specify as and a new local name, or alias, for the type. Listing 7-16 shows another way to write the code in Listing 7-15 by renaming one of the two Result types using as.

In the second use statement, we chose the new name IoResult for the std::io::Result type, which won’t conflict with the Result from std::fmt that we’ve also brought into scope. Listing 7-15 and Listing 7-16 are considered idiomatic, so the choice is up to you!

#### Re-exporting Names with `pub` use
When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.

Listing 7-17 shows the code in Listing 7-11 with use in the root module changed to pub use.

By using pub use, external code can now call the add_to_waitlist function using hosting::add_to_waitlist. If we hadn’t specified pub use, the eat_at_restaurant function could call hosting::add_to_waitlist in its scope, but external code couldn’t take advantage of this new path.

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With pub use, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library.


### Error Handling
Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error. This chapter covers calling panic! first and then talks about returning Result<T, E> values. Additionally, we’ll explore considerations when deciding whether to try to recover from an error or to stop execution.

#### Unrecoverable Errors with panic!
Sometimes, bad things happen in your code, and there’s nothing you can do about it. In these cases, Rust has the panic! macro. When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit. This most commonly occurs when a bug of some kind has been detected and it’s not clear to the programmer how to handle the error.

##### Unwinding the Stack or Aborting in Response to a Panic
By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work. The alternative is to immediately abort, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:

Dev is for development and release is for production

cargo run --release to generate release build

[profile.dev]
panic = 'abort'

[profile.release]
panic = 'abort'

To see backtrace set `RUST_BACKTRACE = 1` environment variable

Powershell
$env:RUST_BACKTRACE = 1

Linux
RUST_BACKTRACE = 1


#### Recoverable Errors with Result

Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

Recall from “Handling Potential Failure with the Result Type” in Chapter 2 that the Result enum is defined as having two variants, Ok and Err, as follows:

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}

```

The T and E are generic type parameters: we’ll discuss generics in more detail in Chapter 10. What you need to know right now is that T represents the type of the value that will be returned in a success case within the Ok variant, and E represents the type of the error that will be returned in a failure case within the Err variant. Because Result has these generic type parameters, we can use the Result type and the functions that the standard library has defined on it in many different situations where the successful value and error value we want to return may differ.


#### Matching on Different Errors
The code in Listing 9-4 will panic! no matter why File::open failed. What we want to do instead is take different actions for different failure reasons: if File::open failed because the file doesn’t exist, we want to create the file and return the handle to the new file. If File::open failed for any other reason—for example, because we didn’t have permission to open the file—we still want the code to panic! in the same way as it did in Listing 9-4. Look at Listing 9-5, which adds an inner match expression.

The type of the value that File::open returns inside the Err variant is io::Error, which is a struct provided by the standard library. This struct has a method kind that we can call to get an io::ErrorKind value. The enum io::ErrorKind is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation. The variant we want to use is ErrorKind::NotFound, which indicates the file we’re trying to open doesn’t exist yet. So we match on f, but we also have an inner match on error.kind().

The condition we want to check in the inner match is whether the value returned by error.kind() is the NotFound variant of the ErrorKind enum. If it is, we try to create the file with File::create. However, because File::create could also fail, we need a second arm in the inner match expression. When the file can’t be created, a different error message is printed. The second arm of the outer match stays the same, so the program panics on any error besides the missing file error.

That’s a lot of match! The match expression is very useful but also very much a primitive. In Chapter 13, you’ll learn about closures; the Result<T, E> type has many methods that accept a closure and are implemented using match expressions. Using those methods will make your code more concise. A more seasoned Rustacean might write this code instead of Listing 9-5:

Although this code has the same behavior as Listing 9-5, it doesn’t contain any match expressions and is cleaner to read. Come back to this example after you’ve read Chapter 13, and look up the unwrap_or_else method in the standard library documentation. Many more of these methods can clean up huge nested match expressions when you’re dealing with errors.


#### Shortcuts for Panic on Error: unwrap and expect

Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well. The Result<T, E> type has many helper methods defined on it to do various tasks. One of those methods, called unwrap, is a shortcut method that is implemented just like the match expression we wrote in Listing 9-4. If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us. Here is an example of unwrap in action:

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

If we run this code without a hello.txt file, we’ll see an error message from the panic! call that the unwrap method makes:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

Another method, expect, which is similar to unwrap, lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier. The syntax of expect looks like this:

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

We use expect in the same way as unwrap: to return the file handle or call the panic! macro. The error message used by expect in its call to panic! will be the parameter that we pass to expect, rather than the default panic! message that unwrap uses. Here’s what it looks like:
```
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

Because this error message starts with the text we specified, Failed to open hello.txt, it will be easier to find where in the code this error message is coming from. If we use unwrap in multiple places, it can take more time to figure out exactly which unwrap is causing the panic because all unwrap calls that panic print the same message.


#### Propagating Errors
When you’re writing a function whose implementation calls something that might fail, instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.

For example, Listing 9-6 shows a function that reads a username from a file. If the file doesn’t exist or can’t be read, this function will return those errors to the code that called this function.


This function can be written in a much shorter way, but we’re going to start by doing a lot of it manually in order to explore error handling; at the end, we’ll show the shorter way. Let’s look at the return type of the function first: Result<String, io::Error>. This means the function is returning a value of the type Result<T, E> where the generic parameter T has been filled in with the concrete type String and the generic type E has been filled in with the concrete type io::Error. If this function succeeds without any problems, the code that calls this function will receive an Ok value that holds a String—the username that this function read from the file. If this function encounters any problems, the code that calls this function will receive an Err value that holds an instance of io::Error that contains more information about what the problems were. We chose io::Error as the return type of this function because that happens to be the type of the error value returned from both of the operations we’re calling in this function’s body that might fail: the File::open function and the read_to_string method.

The body of the function starts by calling the File::open function. Then we handle the Result value returned with a match similar to the match in Listing 9-4, only instead of calling panic! in the Err case, we return early from this function and pass the error value from File::open back to the calling code as this function’s error value. If File::open succeeds, we store the file handle in the variable f and continue.

Then we create a new String in variable s and call the read_to_string method on the file handle in f to read the contents of the file into s. The read_to_string method also returns a Result because it might fail, even though File::open succeeded. So we need another match to handle that Result: if read_to_string succeeds, then our function has succeeded, and we return the username from the file that’s now in s wrapped in an Ok. If read_to_string fails, we return the error value in the same way that we returned the error value in the match that handled the return value of File::open. However, we don’t need to explicitly say return, because this is the last expression in the function.

The code that calls this code will then handle getting either an Ok value that contains a username or an Err value that contains an io::Error. We don’t know what the calling code will do with those values. If the calling code gets an Err value, it could call panic! and crash the program, use a default username, or look up the username from somewhere other than a file, for example. We don’t have enough information on what the calling code is actually trying to do, so we propagate all the success or error information upward for it to handle appropriately.

This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

##### A Shortcut for Propagating Errors: the ? Operator
Listing 9-7 shows an implementation of read_username_from_file that has the same functionality as it had in Listing 9-6, but this implementation uses the ? operator.

The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in Listing 9-6. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

There is a difference between what the match expression from Listing 9-6 does and what the ? operator does: error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as each error type implements the from function to define how to convert itself to the returned error type, the ? operator takes care of the conversion automatically.

In the context of Listing 9-7, the ? at the end of the File::open call will return the value inside an Ok to the variable f. If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code. The same thing applies to the ? at the end of the read_to_string call.

The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the ?, as shown in Listing 9-8.

We’ve moved the creation of the new String in s to the beginning of the function; that part hasn’t changed. Instead of creating a variable f, we’ve chained the call to read_to_string directly onto the result of File::open("hello.txt")?. We still have a ? at the end of the read_to_string call, and we still return an Ok value containing the username in s when both File::open and read_to_string succeed rather than returning errors. The functionality is again the same as in Listing 9-6 and Listing 9-7; this is just a different, more ergonomic way to write it.

Speaking of different ways to write this function, Listing 9-9 shows that there’s a way to make this even shorter.

Reading a file into a string is a fairly common operation, so Rust provides the convenient fs::read_to_string function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it. Of course, using fs::read_to_string doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first.
The ? Operator Can Be Used in Functions That Return Result

The ? operator can be used in functions that have a return type of Result, because it is defined to work in the same way as the match expression we defined in Listing 9-6. The part of the match that requires a return type of Result is return Err(e), so the return type of the function can be a Result to be compatible with this return.

Let’s look at what happens if we use the ? operator in the main function, which you’ll recall has a return type of ():

When we compile this code, we get the following error message:

This error points out that we’re only allowed to use the ? operator in a function that returns Result or Option or another type that implements std::ops::Try. When you’re writing code in a function that doesn’t return one of these types, and you want to use ? when you call other functions that return Result<T, E>, you have two choices to fix this problem. One technique is to change the return type of your function to be Result<T, E> if you have no restrictions preventing that. The other technique is to use a match or one of the Result<T, E> methods to handle the Result<T, E> in whatever way is appropriate.

The main function is special, and there are restrictions on what its return type must be. One valid return type for main is (), and conveniently, another valid return type is Result<T, E>, as shown here:

The `Box<dyn Error>` type is called a trait object, which we’ll talk about in the “Using Trait Objects that Allow for Values of Different Types” section in Chapter 17. For now, you can read `Box<dyn Error>` to mean “any kind of error.” Using ? in a main function with this return type is allowed.

Now that we’ve discussed the details of calling panic! or returning Result, let’s return to the topic of how to decide which is appropriate to use in which cases.

### To panic! or Not to panic!
So how do you decide when you should call panic! and when you should return Result? When code panics, there’s no way to recover. You could call panic! for any error situation, whether there’s a possible way to recover or not, but then you’re making the decision on behalf of the code calling your code that a situation is unrecoverable. When you choose to return a Result value, you give the calling code options rather than making the decision for it. The calling code could choose to attempt to recover in a way that’s appropriate for its situation, or it could decide that an Err value in this case is unrecoverable, so it can call panic! and turn your recoverable error into an unrecoverable one. Therefore, returning Result is a good default choice when you’re defining a function that might fail.

In rare situations, it’s more appropriate to write code that panics instead of returning a Result. Let’s explore why it’s appropriate to panic in examples, prototype code, and tests. Then we’ll discuss situations in which the compiler can’t tell that failure is impossible, but you as a human can. The chapter will conclude with some general guidelines on how to decide whether to panic in library code.

#### Examples, Prototype Code, and Tests
When you’re writing an example to illustrate some concept, having robust error-handling code in the example as well can make the example less clear. In examples, it’s understood that a call to a method like unwrap that could panic is meant as a placeholder for the way you’d want your application to handle errors, which can differ based on what the rest of your code is doing.

Similarly, the unwrap and expect methods are very handy when prototyping, before you’re ready to decide how to handle errors. They leave clear markers in your code for when you’re ready to make your program more robust.

If a method call fails in a test, you’d want the whole test to fail, even if that method isn’t the functionality under test. Because panic! is how a test is marked as a failure, calling unwrap or expect is exactly what should happen.

#### Cases in Which You Have More Information Than the Compiler
It would also be appropriate to call unwrap when you have some other logic that ensures the Result will have an Ok value, but the logic isn’t something the compiler understands. You’ll still have a Result value that you need to handle: whatever operation you’re calling still has the possibility of failing in general, even though it’s logically impossible in your particular situation. If you can ensure by manually inspecting the code that you’ll never have an Err variant, it’s perfectly acceptable to call unwrap. Here’s an example:
```
	use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1".parse().unwrap();
```
We’re creating an IpAddr instance by parsing a hardcoded string. We can see that 127.0.0.1 is a valid IP address, so it’s acceptable to use unwrap here. However, having a hardcoded, valid string doesn’t change the return type of the parse method: we still get a Result value, and the compiler will still make us handle the Result as if the Err variant is a possibility because the compiler isn’t smart enough to see that this string is always a valid IP address. If the IP address string came from a user rather than being hardcoded into the program and therefore did have a possibility of failure, we’d definitely want to handle the Result in a more robust way instead.

#### Guidelines for Error Handling
It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

    -The bad state is not something that’s expected to happen occasionally.
    -Your code after this point needs to rely on not being in this bad state.
    -There’s not a good way to encode this information in the types you use.

If someone calls your code and passes in values that don’t make sense, the best choice might be to call panic! and alert the person using your library to the bug in their code so they can fix it during development. Similarly, panic! is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

However, when failure is expected, it’s more appropriate to return a Result than to make a panic! call. Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. In these cases, returning a Result indicates that failure is an expected possibility that the calling code must decide how to handle.

When your code performs operations on values, your code should verify the values are valid first and panic if the values aren’t valid. This is mostly for safety reasons: attempting to operate on invalid data can expose your code to vulnerabilities. This is the main reason the standard library will call panic! if you attempt an out-of-bounds memory access: trying to access memory that doesn’t belong to the current data structure is a common security problem. Functions often have contracts: their behavior is only guaranteed if the inputs meet particular requirements. Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug and it’s not a kind of error you want the calling code to have to explicitly handle. In fact, there’s no reasonable way for calling code to recover; the calling programmers need to fix the code. Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function.

However, having lots of error checks in all of your functions would be verbose and annoying. Fortunately, you can use Rust’s type system (and thus the type checking the compiler does) to do many of the checks for you. If your function has a particular type as a parameter, you can proceed with your code’s logic knowing that the compiler has already ensured you have a valid value. For example, if you have a type rather than an Option, your program expects to have something rather than nothing. Your code then doesn’t have to handle two cases for the Some and None variants: it will only have one case for definitely having a value. Code trying to pass nothing to your function won’t even compile, so your function doesn’t have to check for that case at runtime. Another example is using an unsigned integer type such as u32, which ensures the parameter is never negative.

#### Creating Custom Types for Validation
Let’s take the idea of using Rust’s type system to ensure we have a valid value one step further and look at creating a custom type for validation. Recall the guessing game in Chapter 2 in which our code asked the user to guess a number between 1 and 100. We never validated that the user’s guess was between those numbers before checking it against our secret number; we only validated that the guess was positive. In this case, the consequences were not very dire: our output of “Too high” or “Too low” would still be correct. But it would be a useful enhancement to guide the user toward valid guesses and have different behavior when a user guesses a number that’s out of range versus when a user types, for example, letters instead.

One way to do this would be to parse the guess as an i32 instead of only a u32 to allow potentially negative numbers, and then add a check for the number being in range, like so:

```
	loop {
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
    }
```
The if expression checks whether our value is out of range, tells the user about the problem, and calls continue to start the next iteration of the loop and ask for another guess. After the if expression, we can proceed with the comparisons between guess and the secret number knowing that guess is between 1 and 100.

However, this is not an ideal solution: if it was absolutely critical that the program only operated on values between 1 and 100, and it had many functions with this requirement, having a check like this in every function would be tedious (and might impact performance).

Instead, we can make a new type and put the validations in a function to create an instance of the type rather than repeating the validations everywhere. That way, it’s safe for functions to use the new type in their signatures and confidently use the values they receive. Listing 9-10 shows one way to define a Guess type that will only create an instance of Guess if the new function receives a value between 1 and 100.
```
	pub struct Guess {
	    value: i32,
	}

	impl Guess {
	    pub fn new(value: i32) -> Guess {
	        if value < 1 || value > 100 {
	            panic!("Guess value must be between 1 and 100, got {}.", value);
	        }

	        Guess { value }
	    }

	    pub fn value(&self) -> i32 {
	        self.value
	    }
	}
```
First, we define a struct named Guess that has a field named value that holds an i32. This is where the number will be stored.

Then we implement an associated function named new on Guess that creates instances of Guess values. The new function is defined to have one parameter named value of type i32 and to return a Guess. The code in the body of the new function tests value to make sure it’s between 1 and 100. If value doesn’t pass this test, we make a panic! call, which will alert the programmer who is writing the calling code that they have a bug they need to fix, because creating a Guess with a value outside this range would violate the contract that Guess::new is relying on. The conditions in which Guess::new might panic should be discussed in its public-facing API documentation; we’ll cover documentation conventions indicating the possibility of a panic! in the API documentation that you create in Chapter 14. If value does pass the test, we create a new Guess with its value field set to the value parameter and return the Guess.

Next, we implement a method named value that borrows self, doesn’t have any other parameters, and returns an i32. This kind of method is sometimes called a getter, because its purpose is to get some data from its fields and return it. This public method is necessary because the value field of the Guess struct is private. It’s important that the value field be private so code using the Guess struct is not allowed to set value directly: code outside the module must use the Guess::new function to create an instance of Guess, thereby ensuring there’s no way for a Guess to have a value that hasn’t been checked by the conditions in the Guess::new function.

A function that has a parameter or returns only numbers between 1 and 100 could then declare in its signature that it takes or returns a Guess rather than an i32 and wouldn’t need to do any additional checks in its body.

#### Summary
Rust’s error handling features are designed to help you write more robust code. The panic! macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values. The Result enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from. You can use Result to tell code that calls your code that it needs to handle potential success or failure as well. Using panic! and Result in the appropriate situations will make your code more reliable in the face of inevitable problems.

Now that you’ve seen useful ways that the standard library uses generics with the Option and Result enums, we’ll talk about how generics work and how you can use them in your code.


#### Returning Types that Implements Traits
By using `impl Summary` for the return type, we specify that the `returns_summarizable` function returns some type that implements the `Summary` trait without naming the concrete type. In this case, `returns_summarizable` returns a `Tweet`, but the code calling this function doesn’t know that.

The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators, which we cover in Chapter 13. Closures and iterators create types that only the compiler knows or types that are very long to specify. The `impl Trait` syntax lets you concisely specify that a function returns some type that implements the `Iterator` trait without needing to write out a very long type.

However, you can only use `impl Trait` if you’re returning a single type. For example, this code that returns either a `NewsArticle` or a `Tweet` with the return type specified as impl Summary wouldn’t work.

Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions around how the `impl Trait` syntax is implemented in the compiler. We’ll cover how to write a function with this behavior in the “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17

#### Fixing the `largest` Function with Trait Bounds
Now that you know how to specify the behavior you want to use using the generic type parameter’s bounds, let’s return to Listing 10-5 to fix the definition of the `largest` function that uses a generic type parameter! Last time we tried to run that code, we received this error:
```
  $ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
  error[E0369]: binary operation `>` cannot be applied to type `T`
   --> src/main.rs:5:17
    |
  5 |         if item > largest {
    |            ---- ^ ------- T
    |            |
    |            T
    |
    = note: `T` might need a bound for `std::cmp::PartialOrd`

  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0369`.
  error: could not compile `chapter10`.

  To learn more, run the command again with --verbose.

```

In the body of `largest` we wanted to compare two values of type `T` using the greater than `(>)` operator. Because that operator is defined as a default method on the standard library trait `std::cmp::PartialOrd`, we need to specify `PartialOrd` in the trait bounds for `T` so the `largest` function can work on slices of any type that we can compare. We don’t need to bring `PartialOrd` into scope because it’s in the prelude. Change the signature of `largest` to look like this:
```
  fn largest<T: PartialOrd>(list: &[T]) -> T {
      let mut largest = list[0];

      for &item in list {
          if item > largest {
              largest = item;
          }
      }

      largest
  }

  fn main() {
      let number_list = vec![34, 50, 25, 100, 65];

      let result = largest(&number_list);
      println!("The largest number is {}", result);

      let char_list = vec!['y', 'm', 'a', 'q'];

      let result = largest(&char_list);
      println!("The largest char is {}", result);
  }

```

This time when we compile the code, we get a different set of errors:
```
  $ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
  error[E0508]: cannot move out of type `[T]`, a non-copy slice
   --> src/main.rs:2:23
    |
  2 |     let mut largest = list[0];
    |                       ^^^^^^^
    |                       |
    |                       cannot move out of here
    |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
    |                       help: consider borrowing here: `&list[0]`

  error[E0507]: cannot move out of a shared reference
   --> src/main.rs:4:18
    |
  4 |     for &item in list {
    |         -----    ^^^^
    |         ||
    |         |data moved here
    |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
    |         help: consider removing the `&`: `item`

  error: aborting due to 2 previous errors

  Some errors have detailed explanations: E0507, E0508.
  For more information about an error, try `rustc --explain E0507`.
  error: could not compile `chapter10`.

  To learn more, run the command again with --verbose.

```

The key line in this error is `cannot move out of type [T], a non-copy slice`. With our non-generic versions of the `largest` function, we were only trying to find the largest `i32` or `char`. As discussed in the “Stack-Only Data: Copy” section in Chapter 4, types like `i32` and `char` that have a known size can be stored on the stack, so they implement the `Copy` trait. But when we made the `largest` function generic, it became possible for the `list` parameter to have types in it that don’t implement the `Copy` trait. Consequently, we wouldn’t be able to move the value out of `list[0]` and into the `largest` variable, resulting in this error.

To call this code with only those types that implement the `Copy` trait, we can add `Copy` to the trait bounds of `T`! Listing 10-15 shows the complete code of a generic `largest` function that will compile as long as the types of the values in the slice that we pass into the function implement the `PartialOrd` and `Copy` traits, like `i32` and `char` do.

```
  fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
      let mut largest = list[0];

      for &item in list {
          if item > largest {
              largest = item;
          }
      }

      largest
  }

  fn main() {
      let number_list = vec![34, 50, 25, 100, 65];

      let result = largest(&number_list);
      println!("The largest number is {}", result);

      let char_list = vec!['y', 'm', 'a', 'q'];

      let result = largest(&char_list);
      println!("The largest char is {}", result);
  }

```

If we don’t want to restrict the `largest` function to the types that implement the `Copy` trait, we could specify that `T` has the trait bound `Clone` instead of `Copy`. Then we could clone each value in the slice when we want the largest function to have ownership. Using the `clone` function means we’re potentially making more heap allocations in the case of types that own heap data like String, and heap allocations can be slow if we’re working with large amounts of data.

Another way we could implement `largest` is for the function to return a reference to a `T` value in the slice. If we change the return type to `&T` instead of `T`, thereby changing the body of the function to return a reference, we wouldn’t need the `Clone` or `Copy` trait bounds and we could avoid heap allocations. Try implementing these alternate solutions on your own!

#### Using Trait Bounds to Conditionally Implement Methods
By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits. For example, the type `Pair<T>` in Listing 10-16 always implements the `new` function. But `Pair<T>` only implements the `cmp_display` method if its inner type `T`implements the `PartialOrd` trait that enables comparison and the `Display` trait that enables printing.

We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called `blanket implementations` and are extensively used in the Rust standard library. For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait. The `impl` block in the standard library looks similar to this code:
```
  impl<T: Display> ToString for T {
    // --snip--
  }

```

Because the standard library has this blanket implementation, we can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait. For example, we can turn integers into their corresponding `String` values like this because integers implement `Display`:
```
  let s = 3.to_string();
```

Blanket implementations appear in the documentation for the trait in the “Implementors” section.

Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior. The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior. In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method. But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics.

Another kind of generic that we’ve already been using is called lifetimes. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be. Let’s look at how lifetimes do that.

#### Validating References with Lifetimes
One detail we didn’t discuss in the “References and Borrowing” section in Chapter 4 is that every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

The concept of lifetimes is somewhat different from tools in other programming languages, arguably making lifetimes Rust’s most distinctive feature. Although we won’t cover lifetimes in their entirety in this chapter, we’ll discuss common ways you might encounter lifetime syntax so you can become familiar with the concepts.

#### Preventing Dangling References with Lifetimes
The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference. Consider the program in Listing 10-17, which has an outer scope and an inner scope.
```
  fn main() {
      {
          let r;

          {
              let x = 5;
              r = &x;
          }

          println!("r: {}", r);
      }
  }

```

Note: The examples in Listings 10-17, 10-18, and 10-24 declare variables without giving them an initial value, so the variable name exists in the outer scope. At first glance, this might appear to be in conflict with Rust’s having no null values. However, if we try to use a variable before giving it a value, we’ll get a compile-time error, which shows that Rust indeed does not allow null values.

The outer scope declares a variable named r with no initial value, and the inner scope declares a variable named x with the initial value of 5. Inside the inner scope, we attempt to set the value of r as a reference to x. Then the inner scope ends, and we attempt to print the value in r. This code won’t compile because the value r is referring to has gone out of scope before we try to use it.

The variable x doesn’t “live long enough.” The reason is that x will be out of scope when the inner scope ends on line 7. But r is still valid for the outer scope; because its scope is larger, we say that it “lives longer.” If Rust allowed this code to work, r would be referencing memory that was deallocated when x went out of scope, and anything we tried to do with r wouldn’t work correctly. So how does Rust determine that this code is invalid? It uses a borrow checker.

#### The Borrow Checker
The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid. Listing 10-18 shows the same code as Listing 10-17 but with annotations showing the lifetimes of the variables.
```
  fn main() {
      {
          let r;                // ---------+-- 'a
                                //          |
          {                     //          |
              let x = 5;        // -+-- 'b  |
              r = &x;           //  |       |
          }                     // -+       |
                                //          |
          println!("r: {}", r); //          |
      }                         // ---------+
  }

```
Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b. As you can see, the inner 'b block is much smaller than the outer 'a lifetime block. At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b. The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.
Listing 10-19 fixes the code so it doesn’t have a dangling reference and compiles without any errors.
```
  fn main() {
      {
          let x = 5;            // ----------+-- 'b
                                //           |
          let r = &x;           // --+-- 'a  |
                                //   |       |
          println!("r: {}", r); //   |       |
                                // --+       |
      }                         // ----------+
  }

```
Here, x has the lifetime 'b, which in this case is larger than 'a. This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.

Now that you know where the lifetimes of references are and how Rust analyzes lifetimes to ensure references will always be valid, let’s explore generic lifetimes of parameters and return values in the context of functions.

#### Lifetime Annotation Syntax
Lifetime annotations don’t change how long any of the references live. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter. Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types. Most people use the name 'a. We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.

Here are some examples: a reference to an i32 without a lifetime parameter, a reference to an i32 that has a lifetime parameter named 'a, and a mutable reference to an i32 that also has the lifetime 'a.
```
  &i32        // a reference
  &'a i32     // a reference with an explicit lifetime
  &'a mut i32 // a mutable reference with an explicit lifetime

```
One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. For example, let’s say we have a function with the parameter first that is a reference to an i32 with lifetime 'a. The function also has another parameter named second that is another reference to an i32 that also has the lifetime 'a. The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.

#### Lifetime Annotations in Function Signatures
Now let’s examine lifetime annotations in the context of the longest function. As with generic type parameters, we need to declare generic lifetime parameters inside angle brackets between the function name and the parameter list. The constraint we want to express in this signature is that all the references in the parameters and the return value must have the same lifetime. We’ll name the lifetime 'a and then add it to each reference, as shown in Listing 10-22.
```
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }

```
This code should compile and produce the result we want when we use it with the main function in Listing 10-20.

The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in. These constraints are what we want Rust to enforce. Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.

When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. Rust can analyze the code within the function without any help. However, when a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually.

When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.

Let’s look at how the lifetime annotations restrict the longest function by passing in references that have different concrete lifetimes. Listing 10-23 is a straightforward example.
```
  fn main() {
      let string1 = String::from("long string is long");

      {
          let string2 = String::from("xyz");
          let result = longest(string1.as_str(), string2.as_str());
          println!("The longest string is {}", result);
      }
  }

  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }

```
In this example, string1 is valid until the end of the outer scope, string2 is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope. Run this code, and you’ll see that the borrow checker approves of this code; it will compile and print The longest string is long string is long.

Next, let’s try an example that shows that the lifetime of the reference in result must be the smaller lifetime of the two arguments. We’ll move the declaration of the result variable outside the inner scope but leave the assignment of the value to the result variable inside the scope with string2. Then we’ll move the println! that uses result outside the inner scope, after the inner scope has ended. The code in Listing 10-24 will not compile.
```
  fn main() {
      let string1 = String::from("long string is long");
      let result;
      {
          let string2 = String::from("xyz");
          result = longest(string1.as_str(), string2.as_str());
      }
      println!("The longest string is {}", result);
  }
```
When we try to compile this code, we’ll get this error:
```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
  error[E0597]: `string2` does not live long enough
   --> src/main.rs:6:44
    |
  6 |         result = longest(string1.as_str(), string2.as_str());
    |                                            ^^^^^^^ borrowed value does not live long enough
  7 |     }
    |     - `string2` dropped here while still borrowed
  8 |     println!("The longest string is {}", result);
    |                                          ------ borrow later used here

  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0597`.
  error: could not compile `chapter10`.

  To learn more, run the command again with --verbose.

```
The error shows that for result to be valid for the println! statement, string2 would need to be valid until the end of the outer scope. Rust knows this because we annotated the lifetimes of the function parameters and return values using the same lifetime parameter 'a.

As humans, we can look at this code and see that string1 is longer than string2 and therefore result will contain a reference to string1. Because string1 has not gone out of scope yet, a reference to string1 will still be valid for the println! statement. However, the compiler can’t see that the reference is valid in this case. We’ve told Rust that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in. Therefore, the borrow checker disallows the code in Listing 10-24 as possibly having an invalid reference.

#### Thinking in Terms of Lifetimes

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

#### Lifetime Elision
You’ve learned that every reference has a lifetime and that you need to specify lifetime parameters for functions or structs that use references. However, in Chapter 4 we had a function in Listing 4-9, which is shown again in Listing 10-26, that compiled without lifetime annotations.
```
  fn first_word(s: &str) -> &str {
      let bytes = s.as_bytes();

      for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
              return &s[0..i];
          }
      }

      &s[..]
  }

  fn main() {
      let my_string = String::from("hello world");

      // first_word works on slices of `String`s
      let word = first_word(&my_string[..]);

      let my_string_literal = "hello world";

      // first_word works on slices of string literals
      let word = first_word(&my_string_literal[..]);

      // Because string literals *are* string slices already,
      // this works too, without the slice syntax!
      let word = first_word(my_string_literal);
  }

```
The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime. At that time, the function signature would have been written like this:
```
  fn first_word<'a>(s: &'a str) -> &'a str {
```
After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotations over and over in particular situations. These situations were predictable and followed a few deterministic patterns. The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

This piece of Rust history is relevant because it’s possible that more deterministic patterns will emerge and be added to the compiler. In the future, even fewer lifetime annotations might be required.

The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

The elision rules don’t provide full inference. If Rust deterministically applies the rules but there is still ambiguity as to what lifetimes the references have, the compiler won’t guess what the lifetime of the remaining references should be. In this case, instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations that specify how the references relate to each other.

Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.

The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`;a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so on.

The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.

The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

Let’s pretend we’re the compiler. We’ll apply these rules to figure out what the lifetimes of the references in the signature of the first_word function in Listing 10-26 are. The signature starts without any lifetimes associated with the references:
```
  fn first_word(s: &str) -> &str {
```
Then the compiler applies the first rule, which specifies that each parameter gets its own lifetime. We’ll call it 'a as usual, so now the signature is this:
```
  fn first_word<'a>(s: &'a str) -> &str {
```
The second rule applies because there is exactly one input lifetime. The second rule specifies that the lifetime of the one input parameter gets assigned to the output lifetime, so the signature is now this:
```
  fn first_word<'a>(s: &'a str) -> &'a str {
```
Now all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature.

Let’s look at another example, this time using the longest function that had no lifetime parameters when we started working with it in Listing 10-21:
```
  fn longest(x: &str, y: &str) -> &str {
```
Let’s apply the first rule: each parameter gets its own lifetime. This time we have two parameters instead of one, so we have two lifetimes:
```
  fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```
You can see that the second rule doesn’t apply because there is more than one input lifetime. The third rule doesn’t apply either, because longest is a function rather than a method, so none of the parameters are self. After working through all three rules, we still haven’t figured out what the return type’s lifetime is. This is why we got an error trying to compile the code in Listing 10-21: the compiler worked through the lifetime elision rules but still couldn’t figure out all the lifetimes of the references in the signature.

Because the third rule really only applies in method signatures, we’ll look at lifetimes in that context next to see why the third rule means we don’t have to annotate lifetimes in method signatures very often.

#### Lifetime Annotations in Method Definitions
When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters shown in Listing 10-11. Where we declare and use the lifetime parameters depends on whether they’re related to the struct fields or the method parameters and return values.

Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent. In addition, the lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures. Let’s look at some examples using the struct named ImportantExcerpt that we defined in Listing 10-25.

First, we’ll use a method named level whose only parameter is a reference to self and whose return value is an i32, which is not a reference to anything:
```
  impl<'a> ImportantExcerpt<'a> {
      fn level(&self) -> i32 {
          3
      }
  }
```
The lifetime parameter declaration after impl and its use after the type name are required, but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.

Here is an example where the third lifetime elision rule applies:
```
  impl<'a> ImportantExcerpt<'a> {
      fn announce_and_return_part(&self, announcement: &str) -> &str {
          println!("Attention please: {}", announcement);
          self.part
      }
  }
```
There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.

#### The Static Lifetime
One special lifetime we need to discuss is 'static, which means that this reference can live for the entire duration of the program. All string literals have the 'static lifetime, which we can annotate as follows:
```
let s: &'static str = "I have a static lifetime.";
```
The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.

You might see suggestions to use the 'static lifetime in error messages. But before specifying 'static as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not. You might consider whether you want it to live that long, even if it could. Most of the time, the problem results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not specifying the 'static lifetime.

#### Generic Type Parameters, Trait Bounds, and Lifetimes Together
Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!
```
  fn main() {
      let string1 = String::from("abcd");
      let string2 = "xyz";

      let result = longest_with_an_announcement(
          string1.as_str(),
          string2,
          "Today is someone's birthday!",
      );
      println!("The longest string is {}", result);
  }

  use std::fmt::Display;

  fn longest_with_an_announcement<'a, T>(
      x: &'a str,
      y: &'a str,
      ann: T,
  ) -> &'a str
  where
      T: Display,
  {
      println!("Announcement! {}", ann);
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }
```
This is the longest function from Listing 10-22 that returns the longer of two string slices. But now it has an extra parameter named ann of the generic type T, which can be filled in by any type that implements the Display trait as specified by the where clause. This extra parameter will be printed before the function compares the lengths of the string slices, which is why the Display trait bound is necessary. Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.

#### Summary
We covered a lot in this chapter! Now that you know about generic type parameters, traits and trait bounds, and generic lifetime parameters, you’re ready to write code without repetition that works in many different situations. Generic type parameters let you apply the code to different types. Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references. And all of this analysis happens at compile time, which doesn’t affect runtime performance!

Believe it or not, there is much more to learn on the topics we discussed in this chapter: Chapter 17 discusses trait objects, which are another way to use traits. There are also more complex scenarios involving lifetime annotations that you will only need in very advanced scenarios; for those, you should read the Rust Reference. But next, you’ll learn how to write tests in Rust so you can make sure your code is working the way it should.

##### Functional Language Features: Iterators and Closures

Rust’s design has taken inspiration from many existing languages and techniques, and one significant influence is *functional programming*. Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth.

In this chapter, we won’t debate the issue of what functional programming is or isn’t but will instead discuss some features of Rust that are similar to features in many languages often referred to as functional.

More specifically, we’ll cover:

  -  *Closures*, a function-like construct you can store in a variable
  -  *Iterators*, a way of processing a series of elements
  -  How to use these two features to improve the I/O project in Chapter 12
  -  The performance of these two features (Spoiler alert: they’re faster than you might think!)

Other Rust features, such as pattern matching and enums, which we’ve covered in other chapters, are influenced by the functional style as well. Mastering closures and iterators is an important part of writing idiomatic, fast Rust code, so we’ll devote this entire chapter to them.

#### Closures: Anonymous Functions that Can Capture Their Environment
Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re defined. We’ll demonstrate how these closure features allow for code reuse and behavior customization.

#### Creating an Abstraction of Behavior with Closures
Let’s work on an example of a situation in which it’s useful to store a closure to be executed later. Along the way, we’ll talk about the syntax of closures, type inference, and traits.

Consider this hypothetical situation: we work at a startup that’s making an app to generate custom exercise workout plans. The backend is written in Rust, and the algorithm that generates the workout plan takes into account many factors, such as the app user’s age, body mass index, exercise preferences, recent workouts, and an intensity number they specify. The actual algorithm used isn’t important in this example; what’s important is that this calculation takes a few seconds. We want to call this algorithm only when we need to and only call it once so we don’t make the user wait more than necessary.

We’ll simulate calling this hypothetical algorithm with the function `simulated_expensive_calculation` shown in Listing 13-1, which will print `calculating slowly...`, wait for two seconds, and then return whatever number we passed in.

We want to define code in one place in our program, but only execute that code where we actually need the result. This is a use case for closures!

#### Closure Type Inference and Annotation
Closures don’t require you to annotate the types of the parameters or the return value like fn functions do. Type annotations are required on functions because they’re part of an explicit interface exposed to your users. Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns. But closures aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.

Closures are usually short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler is reliably able to infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables.

Making programmers annotate the types in these small, anonymous functions would be annoying and largely redundant with the information the compiler already has available.

As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary. Annotating the types for the closure we defined in Listing 13-5 would look like the definition shown in Listing 13-7.
```
  let expensive_closure = |num: u32| -> u32 {
      println!("calculating slowly...");
      thread::sleep(Duration::from_secs(2));
      num
  };
```
With type annotations added, the syntax of closures looks more similar to the syntax of functions. The following is a vertical comparison of the syntax for the definition of a function that adds 1 to its parameter and a closure that has the same behavior. We’ve added some spaces to line up the relevant parts. This illustrates how closure syntax is similar to function syntax except for the use of pipes and the amount of syntax that is optional:
```
  fn  add_one_v1   (x: u32) -> u32 { x + 1 }
  let add_one_v2 = |x: u32| -> u32 { x + 1 };
  let add_one_v3 = |x|             { x + 1 };
  let add_one_v4 = |x|               x + 1  ;
```
The first line shows a function definition, and the second line shows a fully annotated closure definition. The third line removes the type annotations from the closure definition, and the fourth line removes the brackets, which are optional because the closure body has only one expression. These are all valid definitions that will produce the same behavior when they’re called. Calling the closures is required for add_one_v3 and add_one_v4 to be able to compile because the types will be inferred from their usage.

Closure definitions will have one concrete type inferred for each of their parameters and for their return value. For instance, Listing 13-8 shows the definition of a short closure that just returns the value it receives as a parameter. This closure isn’t very useful except for the purposes of this example. Note that we haven’t added any type annotations to the definition: if we then try to call the closure twice, using a String as an argument the first time and a u32 the second time, we’ll get an error.
```
  fn main() {
      let example_closure = |x| x;

      let s = example_closure(String::from("hello"));
      let n = example_closure(5);
  }
```
The compiler gives us this error:
```
  $ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
  error[E0308]: mismatched types
   --> src/main.rs:5:29
    |
  5 |     let n = example_closure(5);
    |                             ^
    |                             |
    |                             expected struct `std::string::String`, found integer
    |                             help: try using a conversion method: `5.to_string()`

  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0308`.
  error: could not compile `closure-example`.

  To learn more, run the command again with --verbose.
```
The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String. Those types are then locked in to the closure in example_closure, and we get a type error if we try to use a different type with the same closure.

#### Storing Closures Using Generic Parameters and the Fn Traits
Let’s return to our workout generation app. In Listing 13-6, our code was still calling the expensive calculation closure more times than it needed to. One option to solve this issue is to save the result of the expensive closure in a variable for reuse and use the variable in each place we need the result, instead of calling the closure again. However, this method could result in a lot of repeated code.

Fortunately, another solution is available to us. We can create a struct that will hold the closure and the resulting value of calling the closure. The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. You may know this pattern as *memoization* or *lazy evaluation*.

To make a struct that holds a closure, we need to specify the type of the closure, because a struct definition needs to know the types of each of its fields. Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature, their types are still considered different. To define structs, enums, or function parameters that use closures, we use generics and trait bounds, as we discussed in Chapter 10.

The Fn traits are provided by the standard library. All closures implement at least one of the traits: Fn, FnMut, or FnOnce. We’ll discuss the difference between these traits in the “Capturing the Environment with Closures” section; in this example, we can use the Fn trait.

We add types to the Fn trait bound to represent the types of the parameters and return values the closures must have to match this trait bound. In this case, our closure has a parameter of type u32 and returns a u32, so the trait bound we specify is Fn(u32) -> u32.

Listing 13-9 shows the definition of the Cacher struct that holds a closure and an optional result value.
```
  struct Cacher<T>
  where
      T: Fn(u32) -> u32,
  {
      calculation: T,
      value: Option<u32>,
  }
```
The Cacher struct has a calculation field of the generic type T. The trait bounds on T specify that it’s a closure by using the Fn trait. Any closure we want to store in the calculation field must have one u32 parameter (specified within the parentheses after Fn) and must return a u32 (specified after the ->).

    Note: Functions can implement all three of the Fn traits too. If what we want to do doesn’t require capturing a value from the environment, we can use a function rather than a closure where we need something that implements an Fn trait.

The value field is of type Option<u32>. Before we execute the closure, value will be None. When code using a Cacher asks for the result of the closure, the Cacher will execute the closure at that time and store the result within a Some variant in the value field. Then if the code asks for the result of the closure again, instead of executing the closure again, the Cacher will return the result held in the Some variant.

The logic around the value field we’ve just described is defined in Listing 13-10.
```
  impl<T> Cacher<T>
  where
      T: Fn(u32) -> u32,
  {
      fn new(calculation: T) -> Cacher<T> {
          Cacher {
              calculation,
              value: None,
          }
      }

      fn value(&mut self, arg: u32) -> u32 {
          match self.value {
              Some(v) => v,
              None => {
                  let v = (self.calculation)(arg);
                  self.value = Some(v);
                  v
              }
          }
      }
  }
```
We want Cacher to manage the struct fields’ values rather than letting the calling code potentially change the values in these fields directly, so these fields are private.

The Cacher::new function takes a generic parameter T, which we’ve defined as having the same trait bound as the Cacher struct. Then Cacher::new returns a Cacher instance that holds the closure specified in the calculation field and a None value in the value field, because we haven’t executed the closure yet.

When the calling code needs the result of evaluating the closure, instead of calling the closure directly, it will call the value method. This method checks whether we already have a resulting value in self.value in a Some; if we do, it returns the value within the Some without executing the closure again.

If self.value is None, the code calls the closure stored in self.calculation, saves the result in self.value for future use, and returns the value as well.

Listing 13-11 shows how we can use this Cacher struct in the function generate_workout from Listing 13-6.

```
  fn generate_workout(intensity: u32, random_number: u32) {
      let mut expensive_result = Cacher::new(|num| {
          println!("calculating slowly...");
          thread::sleep(Duration::from_secs(2));
          num
      });

      if intensity < 25 {
          println!("Today, do {} pushups!", expensive_result.value(intensity));
          println!("Next, do {} situps!", expensive_result.value(intensity));
      } else {
          if random_number == 3 {
              println!("Take a break today! Remember to stay hydrated!");
          } else {
              println!(
                  "Today, run for {} minutes!",
                  expensive_result.value(intensity)
              );
          }
      }
  }
```
Instead of saving the closure in a variable directly, we save a new instance of Cacher that holds the closure. Then, in each place we want the result, we call the value method on the Cacher instance. We can call the value method as many times as we want, or not call it at all, and the expensive calculation will be run a maximum of once.

Try running this program with the main function from Listing 13-2. Change the values in the simulated_user_specified_value and simulated_random_number variables to verify that in all the cases in the various if and else blocks, calculating slowly... appears only once and only when needed. The Cacher takes care of the logic necessary to ensure we aren’t calling the expensive calculation more than we need to so generate_workout can focus on the business logic.

#### Limitations of the Cacher Implementation
Caching values is a generally useful behavior that we might want to use in other parts of our code with different closures. However, there are two problems with the current implementation of Cacher that would make reusing it in different contexts difficult.

The first problem is that a Cacher instance assumes it will always get the same value for the parameter arg to the value method. That is, this test of Cacher will fail:
```
  #[test]
  fn call_with_different_values() {
      let mut c = Cacher::new(|a| a);

      let v1 = c.value(1);
      let v2 = c.value(2);

      assert_eq!(v2, 2);
  }
```
This test creates a new Cacher instance with a closure that returns the value passed into it. We call the value method on this Cacher instance with an arg value of 1 and then an arg value of 2, and we expect the call to value with the arg value of 2 to return 2.

Run this test with the Cacher implementation in Listing 13-9 and Listing 13-10, and the test will fail on the assert_eq! with this message:
```
  $ cargo test
   Compiling cacher v0.1.0 (file:///projects/cacher)
    Finished test [unoptimized + debuginfo] target(s) in 0.72s
     Running target/debug/deps/cacher-4116485fb32b3fff

  running 1 test
  test tests::call_with_different_values ... FAILED

  failures:

  ---- tests::call_with_different_values stdout ----
  thread 'main' panicked at 'assertion failed: `(left == right)`
    left: `1`,
   right: `2`', src/lib.rs:43:9
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


  failures:
      tests::call_with_different_values

  test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

  error: test failed, to rerun pass '--lib'
```
The problem is that the first time we called c.value with 1, the Cacher instance saved Some(1) in self.value. Thereafter, no matter what we pass in to the value method, it will always return 1.

Try modifying Cacher to hold a hash map rather than a single value. The keys of the hash map will be the arg values that are passed in, and the values of the hash map will be the result of calling the closure on that key. Instead of looking at whether self.value directly has a Some or a None value, the value function will look up the arg in the hash map and return the value if it’s present. If it’s not present, the Cacher will call the closure and save the resulting value in the hash map associated with its arg value.

The second problem with the current Cacher implementation is that it only accepts closures that take one parameter of type u32 and return a u32. We might want to cache the results of closures that take a string slice and return usize values, for example. To fix this issue, try introducing more generic parameters to increase the flexibility of the Cacher functionality.

#### Capturing the Environment with Closures
In the workout generator example, we only used closures as inline anonymous functions. However, closures have an additional capability that functions don’t have: they can capture their environment and access variables from the scope in which they’re defined.

#### Fearless Concurrency
Handling concurrent programming safely and efficiently is another of Rust’s major goals. *Concurrent programming*, where different parts of a program execute independently, and *parallel programming*, where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors. Historically, programming in these contexts has been difficult and error prone: Rust hopes to change that.

Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems were two separate challenges to be solved with different methods. Over time, the team discovered that the ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems! By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors. Therefore, rather than making you spend lots of time trying to reproduce the exact circumstances under which a runtime concurrency bug occurs, incorrect code will refuse to compile and present an error explaining the problem. As a result, you can fix your code while you’re working on it rather than potentially after it has been shipped to production. We’ve nicknamed this aspect of Rust fearless concurrency. Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.

    Note: For simplicity’s sake, we’ll refer to many of the problems as concurrent rather than being more precise by saying concurrent and/or parallel. If this book were about concurrency and/or parallelism, we’d be more specific. For this chapter, please mentally substitute concurrent and/or parallel whenever we use concurrent.
Many languages are dogmatic about the solutions they offer for handling concurrent problems. For example, Erlang has elegant functionality for message-passing concurrency but has only obscure ways to share state between threads. Supporting only a subset of possible solutions is a reasonable strategy for higher-level languages, because a higher-level language promises benefits from giving up some control to gain abstractions. However, lower-level languages are expected to provide the solution with the best performance in any given situation and have fewer abstractions over the hardware. Therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements.

Here are the topics we’ll cover in this chapter:

  -  How to create threads to run multiple pieces of code at the same time
  -  *Message-passing* concurrency, where channels send messages between threads
  -  *Shared-state* concurrency, where multiple threads have access to some piece of data
  -  The `Sync` and `Send` traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library

#### Using Threads to Run Code Simultaneously
n most current operating systems, an executed program’s code is run in a *process*, and the operating system manages multiple processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called *threads*.

Splitting the computation in your program into multiple threads can improve performance because the program does multiple tasks at the same time, but it also adds complexity. Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:

  -  Race conditions, where threads are accessing data or resources in an inconsistent order
  -  Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
  -  Bugs that happen only in certain situations and are hard to reproduce and fix reliably

Rust attempts to mitigate the negative effects of using threads, but programming in a multithreaded context still takes careful thought and requires a code structure that is different from that in programs running in a single thread.

Programming languages implement threads in a few different ways. Many operating systems provide an API for creating new threads. This model where a language calls the operating system APIs to create threads is sometimes called *1:1*, meaning one operating system thread per one language thread.

Many programming languages provide their own special implementation of threads. Programming language-provided threads are known as *green* threads, and languages that use these green threads will execute them in the context of a different number of operating system threads. For this reason, the green-threaded model is called the *M:N* model: there are `M` green threads per `N` operating system threads, where `M` and `N` are not necessarily the same number.

Each model has its own advantages and trade-offs, and the trade-off most important to Rust is runtime support. *Runtime* is a confusing term and can have different meanings in different contexts.

In this context, by *runtime* we mean code that is included by the language in every binary. This code can be large or small depending on the language, but every non-assembly language will have some amount of runtime code. For that reason, colloquially when people say a language has “no runtime,” they often mean “small runtime.” Smaller runtimes have fewer features but have the advantage of resulting in smaller binaries, which make it easier to combine the language with other languages in more contexts. Although many languages are okay with increasing the runtime size in exchange for more features, Rust needs to have nearly no runtime and cannot compromise on being able to call into C to maintain performance.

The green-threading M:N model requires a larger language runtime to manage threads. As such, the Rust standard library only provides an implementation of 1:1 threading. Because Rust is such a low-level language, there are crates that implement M:N threading if you would rather trade overhead for aspects such as more control over which threads run when and lower costs of context switching, for example.

Now that we’ve defined threads in Rust, let’s explore how to use the thread-related API provided by the standard library.

#### Creating a New Thread with spawn
To create a new thread, we call the thread::spawn function and pass it a closure (we talked about closures in Chapter 13) containing the code we want to run in the new thread. The example in Listing 16-1 prints some text from a main thread and other text from a new thread:
```
  use std::thread;
  use std::time::Duration;

  fn main() {
      thread::spawn(|| {
          for i in 1..10 {
              println!("hi number {} from the spawned thread!", i);
              thread::sleep(Duration::from_millis(1));
          }
      });

      for i in 1..5 {
          println!("hi number {} from the main thread!", i);
          thread::sleep(Duration::from_millis(1));
      }
  }
```
Note that with this function, the new thread will be stopped when the main thread ends, whether or not it has finished running. The output from this program might be a little different every time, but it will look similar to the following:
```
  hi number 1 from the main thread!
  hi number 1 from the spawned thread!
  hi number 2 from the main thread!
  hi number 2 from the spawned thread!
  hi number 3 from the main thread!
  hi number 3 from the spawned thread!
  hi number 4 from the main thread!
  hi number 4 from the spawned thread!
  hi number 5 from the spawned thread!
```
The calls to `thread::sleep` force a thread to stop its execution for a short duration, allowing a different thread to run. The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads. In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code. And even though we told the spawned thread to print until `i` is 9, it only got to 5 before the main thread shut down.

If you run this code and only see output from the main thread, or don’t see any overlap, try increasing the numbers in the ranges to create more opportunities for the operating system to switch between the threads.

#### Waiting for All Threads to Finish Using `join` Handles

The code in Listing 16-1 not only stops the spawned thread prematurely most of the time due to the main thread ending, but also can’t guarantee that the spawned thread will get to run at all. The reason is that there is no guarantee on the order in which threads run!

We can fix the problem of the spawned thread not getting to run, or not getting to run completely, by saving the return value of `thread::spawn` in a variable. The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned value that, when we call the `join` method on it, will wait for its thread to finish. Listing 16-2 shows how to use the `JoinHandle` of the thread we created in Listing 16-1 and call `join` to make sure the spawned thread finishes before `main` exits:
```
  use std::thread;
  use std::time::Duration;

  fn main() {
      let handle = thread::spawn(|| {
          for i in 1..10 {
              println!("hi number {} from the spawned thread!", i);
              thread::sleep(Duration::from_millis(1));
          }
      });

      for i in 1..5 {
          println!("hi number {} from the main thread!", i);
          thread::sleep(Duration::from_millis(1));
      }

      handle.join().unwrap();
  }
```
Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates. *Blocking* a thread means that thread is prevented from performing work or exiting. Because we’ve put the call to `join` after the main thread’s `for` loop, running Listing 16-2 should produce output similar to this:
```
  hi number 1 from the main thread!
  hi number 2 from the main thread!
  hi number 1 from the spawned thread!
  hi number 3 from the main thread!
  hi number 2 from the spawned thread!
  hi number 4 from the main thread!
  hi number 3 from the spawned thread!
  hi number 4 from the spawned thread!
  hi number 5 from the spawned thread!
  hi number 6 from the spawned thread!
  hi number 7 from the spawned thread!
  hi number 8 from the spawned thread!
  hi number 9 from the spawned thread!
```
The two threads continue alternating, but the main thread waits because of the call to `handle.join()` and does not end until the spawned thread is finished.

But let’s see what happens when we instead move `handle.join()` before the `for` loop in `main`, like this:
```
  use std::thread;
  use std::time::Duration;

  fn main() {
      let handle = thread::spawn(|| {
          for i in 1..10 {
              println!("hi number {} from the spawned thread!", i);
              thread::sleep(Duration::from_millis(1));
          }
      });

      handle.join().unwrap();

      for i in 1..5 {
          println!("hi number {} from the main thread!", i);
          thread::sleep(Duration::from_millis(1));
      }
  }
```
The main thread will wait `for` the spawned thread to finish and then run its for loop, so the output won’t be interleaved anymore, as shown here:
```
  hi number 1 from the spawned thread!
  hi number 2 from the spawned thread!
  hi number 3 from the spawned thread!
  hi number 4 from the spawned thread!
  hi number 5 from the spawned thread!
  hi number 6 from the spawned thread!
  hi number 7 from the spawned thread!
  hi number 8 from the spawned thread!
  hi number 9 from the spawned thread!
  hi number 1 from the main thread!
  hi number 2 from the main thread!
  hi number 3 from the main thread!
  hi number 4 from the main thread!
```
Small details, such as where `join` is called, can affect whether or not your threads run at the same time.

#### Using `move` Closures with Threads
The `move` closure is often used alongside `thread::spawn` because it allows you to use data from one thread in another thread.

In Chapter 13, we mentioned we can use the `move` keyword before the parameter list of a closure to force the closure to take ownership of the values it uses in the environment. This technique is especially useful when creating new threads in order to transfer ownership of values from one thread to another.

Notice in Listing 16-1 that the closure we pass to `thread::spawn` takes no arguments: we’re not using any data from the main thread in the spawned thread’s code. To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs. Listing 16-3 shows an attempt to create a vector in the main thread and use it in the spawned thread. However, this won’t yet work, as you’ll see in a moment.
```
  use std::thread;

  fn main() {
      let v = vec![1, 2, 3];

      let handle = thread::spawn(|| {
          println!("Here's a vector: {:?}", v);
      });

      handle.join().unwrap();
  }
```
The closure uses `v`, so it will capture `v` and make it part of the closure’s environment. Because `thread::spawn` runs this closure in a new thread, we should be able to access `v` inside that new thread. But when we compile this example, we get the following error:
```
  $ cargo run
   Compiling threads v0.1.0 (file:///projects/threads)
  error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
   --> src/main.rs:6:32
    |
  6 |     let handle = thread::spawn(|| {
    |                                ^^ may outlive borrowed value `v`
  7 |         println!("Here's a vector: {:?}", v);
    |                                           - `v` is borrowed here
    |
  note: function requires argument type to outlive `'static`
   --> src/main.rs:6:18
    |
  6 |       let handle = thread::spawn(|| {
    |  __________________^
  7 | |         println!("Here's a vector: {:?}", v);
  8 | |     });
    | |______^
  help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
    |
  6 |     let handle = thread::spawn(move || {
    |                                ^^^^^^^

  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0373`.
  error: could not compile `threads`.

  To learn more, run the command again with --verbose.
```
Rust infers how to capture `v`, and because `println!` only needs a reference to `v`, the closure tries to borrow `v`. However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to `v` will always be valid.

Listing 16-4 provides a scenario that’s more likely to have a reference to `v` that won’t be valid:
```
  use std::thread;

  fn main() {
      let v = vec![1, 2, 3];

      let handle = thread::spawn(|| {
          println!("Here's a vector: {:?}", v);
      });

      drop(v); // oh no!

      handle.join().unwrap();
  }
```
If we were allowed to run this code, there’s a possibility the spawned thread would be immediately put in the background without running at all. The spawned thread has a reference to `v` inside, but the main thread immediately drops `v`, using the `drop` function we discussed in Chapter 15. Then, when the spawned thread starts to execute, `v` is no longer valid, so a reference to it is also invalid. Oh no!

To fix the compiler error in Listing 16-3, we can use the error message’s advice:
```
  help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
    |
  6 |     let handle = thread::spawn(move || {
    |                                ^^^^^^^
```
By adding the `move` keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values. The modification to Listing 16-3 shown in Listing 16-5 will compile and run as we intend:
```
  use std::thread;

  fn main() {
      let v = vec![1, 2, 3];

      let handle = thread::spawn(move || {
          println!("Here's a vector: {:?}", v);
      });

      handle.join().unwrap();
  }
```
What would happen to the code in Listing 16-4 where the main thread called `drop` if we use a `move` closure? Would move fix that case? Unfortunately, no; we would get a different error because what Listing 16-4 is trying to do isn’t allowed for a different reason. If we added `move` to the closure, we would move `v` into the closure’s environment, and we could no longer call `drop` on it in the main thread. We would get this compiler error instead:
```
  $ cargo run
   Compiling threads v0.1.0 (file:///projects/threads)
  error[E0382]: use of moved value: `v`
    --> src/main.rs:10:10
     |
  4  |     let v = vec![1, 2, 3];
     |         - move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
  5  | 
  6  |     let handle = thread::spawn(move || {
     |                                ------- value moved into closure here
  7  |         println!("Here's a vector: {:?}", v);
     |                                           - variable moved due to use in closure
  ...
  10 |     drop(v); // oh no!
     |          ^ value used here after move

  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0382`.
  error: could not compile `threads`.

  To learn more, run the command again with --verbose.
```
Rust’s ownership rules have saved us again! We got an error from the code in Listing 16-3 because Rust was being conservative and only borrowing `v` for the thread, which meant the main thread could theoretically invalidate the spawned thread’s reference. By telling Rust to move ownership of `v` to the spawned thread, we’re guaranteeing Rust that the main thread won’t use `v` anymore. If we change Listing 16-4 in the same way, we’re then violating the ownership rules when we try to use `v` in the main thread. The `move` keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate the ownership rules.

With a basic understanding of threads and the thread API, let’s look at what we can *do* with threads.