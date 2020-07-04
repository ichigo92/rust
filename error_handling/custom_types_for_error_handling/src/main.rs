use custom_types_for_error_handling::cal;


fn main() {
    cal::its_positive(8);
    // Rust compiler throws an error which shows we do not need to do error handling
    // cal::its_positive(-9);
}
