use super::*;

use std::path::Path;

#[test]
fn no_args_x_error() {
    let args = vec!["rosalind".to_string()];
    let c = Config::new(args.into_iter());
    assert!(c.is_err());
}

#[test]
fn one_arg_x_error() {
    let args = vec!["rosalind".to_string(), "file_path".to_string()];
    let c = Config::new(args.into_iter());
    assert!(c.is_err());
}

#[test]
fn two_args_x_to_file_path_and_problem() {
    let args = vec!["rosalind".to_string(),"file_path".to_string(), "problem".to_string()];
    if let Ok(c) = Config::new(args.into_iter()) {
        assert_eq!(Path::new("file_path"), c.file_path);
        assert_eq!("problem", c.problem);
        assert!(c.threads.is_none());
    } else {
        assert!(false);
    }
}

#[test]
fn three_args_x_to_file_path_and_problem_and_threads() {
    let args = vec!["rosalind".to_string(), "file_path".to_string(), "problem".to_string(), "4".to_string()];
    if let Ok(c) = Config::new(args.into_iter()) {
        assert_eq!(Path::new("file_path"), c.file_path);
        assert_eq!("problem", c.problem);
        assert!(c.threads.is_some());
        assert_eq!(4, c.threads.unwrap());
    } else {
        assert!(false);
    }
}

#[test]
fn zero_threads_arg_x_error() {
    let args = vec!["rosalind".to_string(), "file_path".to_string(), "problem".to_string(), "0".to_string()];
    let c = Config::new(args.into_iter());
    assert!(c.is_err());
}

#[test]
fn invalid_threads_arg_x_error() {
    let args = vec!["rosalind".to_string(), "file_path".to_string(), "problem".to_string(), "a".to_string()];
    let c = Config::new(args.into_iter());
    assert!(c.is_err());
}