use gtk::prelude::*;

fn main() {
    // ANCHOR: i32
    // Store `i32` as `Variant`
    let integer_variant = 10.to_variant();

    // Retrieve `i32` from `Variant`
    let integer = integer_variant
        .get::<i32>()
        .expect("The variant needs to be of type `i32`.");

    // Check if the retrieved value is correct
    assert_eq!(integer, 10);
    // ANCHOR_END: i32

    // ANCHOR: vec
    let variant = vec!["Hello", "there!"].to_variant();
    assert_eq!(variant.n_children(), 2);
    let vec = &variant
        .get::<Vec<String>>()
        .expect("The variant needs to be of type `String`.");
    assert_eq!(vec[0], "Hello");
    // ANCHOR_END: vec
}
