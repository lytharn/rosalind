use super::*;

#[test]
fn no_args_x_error() {
    let args = vec![String::from("rosalind")];
    let c = Config::new(&args);
    assert!(c.is_err());
}

#[test]
fn first_arg_x_to_problem() {
    let args = vec![String::from("rosalind"), String::from("problem")];
    if let Ok(c) = Config::new(&args) {
        assert_eq!("problem", c.problem);
    } else {
        assert!(false);
    }
}