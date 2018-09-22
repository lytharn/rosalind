use super::*;

#[test]
fn empty_string_x_zero_result() {
    assert_eq!(run("".as_bytes()), "0 0 0 0");
}

#[test]
fn no_dna_character_x_zero_result() {
    assert_eq!(run("actgBEKUF#$%^&*".as_bytes()), "0 0 0 0");
}

#[test]
fn characters_x_num_dna_characters() {
    assert_eq!(run("ACGT".as_bytes()), "1 1 1 1");
    assert_eq!(run("TGCATGGCCCAAAA".as_bytes()), "5 4 3 2");
    assert_eq!(run("actg%^@#jBEh_CGTA".as_bytes()), "1 1 1 1");
}