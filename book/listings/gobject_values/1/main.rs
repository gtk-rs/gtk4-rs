use gtk::prelude::*;

fn main() {
    // Convert `i32` to `Value`
    let number_value = 10.to_value();

    // Retrieve `i32` again from `Value`
    let number = number_value
        .get::<i32>()
        .expect("The value needs to be of type `i32`.");
    assert_eq!(number, 10);
}
