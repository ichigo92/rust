fn main() {
    let v = vec![1, 2, 3];

    // This error points at a file we didn’t write, libcore/slice/mod.rs.
    // That’s the implementation of slice in the Rust source code.
    // The code that gets run when we use [] on our vector v is in libcore/slice/mod.rs, and that is where the panic! is actually happening.
    v[99];
}
