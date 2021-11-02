#![allow(unused)]
use super::char_shift;
pub(crate) fn decrypt(cypher: &str, key: i8) -> String {
    let mut data = String::with_capacity(cypher.len());
    for ch in cypher.chars() {
        data.push(char_shift(ch, -key));
    }
    data
}
pub(crate) fn encrypt(data: &str, shift: i8) -> String {
    if shift == 0 || shift > 25 {
        println!(
            "Cannot apply caesar cipher with a shift value of {} ",
            shift
        );
        return String::from(data);
    }
    let mut cypher = String::with_capacity(data.len());
    // There is a mode of operation and a permutation
    // The mode of operation here is to go through each letter of this string
    // and the permutation is to shift by a certain amount
    for ch in data.chars() {
        cypher.push(char_shift(ch, shift));
    }
    cypher
}
