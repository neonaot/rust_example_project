mod printers;

use rust_example_project;
use crate::printers::print_five;
// use rust_idea::get_two; // `get_two` is private fun. Uncomment it, use quick fix

fn main() {
    let five = rust_example_project::get_five();
    WrongStyle();
    println!("{}", five);
    print_five();
}

fn WrongStyle() {
    println!("is a style!");
}

