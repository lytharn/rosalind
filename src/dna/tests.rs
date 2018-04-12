use super::*;

#[test]
fn empty_string_x_zero_result() {
    assert_eq!(run(""), "0 0 0 0");
}

#[test]
fn no_dna_character_x_zero_result() {
    assert_eq!(run("actgBEKUF#$%^&*"), "0 0 0 0");
}

#[test]
fn characters_x_num_dna_characters() {
    assert_eq!(run("ACGT"), "1 1 1 1");
    assert_eq!(run("TGCATGGCCCAAAA"), "5 4 3 2");
    assert_eq!(run("actg%^@#jBEh_CGTA"), "1 1 1 1");
}