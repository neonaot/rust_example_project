use rust_example_project::{get_five, get_rand_digit};

#[test]
fn should_pass() {
    assert_eq!(5, get_five());
}


#[test]
fn should_not_pass() {
    assert_eq!(6, get_five());
}

#[test]
fn check_random(){
    let r = get_rand_digit();
    assert!(r < 10);
}