use gtk::prelude::*;

fn main() {
    // ANCHOR: i32
    // Store `Option<String>` as `Value`
    let integer_value = 10.to_value();

    // Retrieve `String` from `Value`
    let integer = integer_value
        .get::<i32>()
        .expect("The value needs to be of type `i32`.");
    assert_eq!(integer, 10);
    // ANCHOR_END: i32

    // ANCHOR: string
    // Store string as `Value`
    let string_value = "Hello!".to_value();

    // Retrieve `String` from `Value`
    let string = string_value
        .get::<String>()
        .expect("The value needs to be of type `String`.");
    assert_eq!(string, "Hello!");
    // ANCHOR_END: string

    // ANCHOR: string_none
    // Store `Option<String>` as `Value`
    let string_none_value = None::<String>.to_value();

    // Retrieve `String` from `Value`
    let string_none = string_none_value
        .get::<Option<String>>()
        .expect("The value needs to be of type `String`.");
    assert_eq!(string_none, None);
    // ANCHOR_END: string_none
}
