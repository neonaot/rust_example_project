pub fn get_five() -> i32 {
    return 5;
}


///This function returns seven
///```
/// use rust_example_project::get_seven;
/// assert_eq!(7, get_seven())
/// ```
pub fn get_seven() -> u8 {
    return 7;
}

fn get_two() -> i32 {
    return 2;
}


pub fn get_rand_digit() -> u8{
    use rand::Rng;
    let mut generator = rand::thread_rng();
    let t = generator.gen_range( 0..10);
    t
}