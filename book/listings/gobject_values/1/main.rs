use gtk::prelude::*;

fn main() {
    let number_value = 10.to_value();
    let string_value = "Hello!".to_value();

    // Retrieve `i32` from `number_value`
    let number = number_value
        .get_some::<i32>()
        .expect("The value needs to be of type `i32`.");
    assert_eq!(number, 10);

    // Retrieve `String` from `string_value`
    let string = string_value
        .get::<String>()
        .expect("The value needs to be of type `String`.")
        .expect("The value needs to be `Some`.");
    assert_eq!(string, "Hello!");
}
