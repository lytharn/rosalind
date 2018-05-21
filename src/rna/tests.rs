use super::*;

#[test]
fn empty_string_x_empty_string() {
    assert_eq!(run(""), "");
}

#[test]
fn dna_characters_x_rna_characters() {
    assert_eq!(run("TATCGT"), "UAUCGU");
    assert_eq!(run("TTTT"), "UUUU");
    assert_eq!(run("CGACGA"), "CGACGA");
}