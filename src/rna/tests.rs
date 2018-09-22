use super::*;

#[test]
fn empty_string_x_empty_string() {
    assert_eq!(run("".as_bytes()), "");
}

#[test]
fn dna_characters_x_rna_characters() {
    assert_eq!(run("TATCGT".as_bytes()), "UAUCGU");
    assert_eq!(run("TTTT".as_bytes()), "UUUU");
    assert_eq!(run("CGACGA".as_bytes()), "CGACGA");
}