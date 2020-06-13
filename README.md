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