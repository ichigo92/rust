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