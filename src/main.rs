use std::vec;

use rand::{thread_rng, Rng};

fn main() {
    assert_eq!('c', char_shift('z', 3));
    let names = vec!["Joseph", "Matthew", "Jesus"];
    let mut rng = thread_rng();
    let shift = rng.gen_range(1..=25);
    println!("Caesar shift value generated: {}", shift);
    for name in names {
        let encrypted = encrypt_caesar(name, shift);
        println!("{} encrypted to {}", name, encrypted);
        println!("Unencrypting {}: {}\n", decrypt_caesar(&encrypted, key: shift));
    }
    
}
fn decrypt_caesar(cypher: &str, key: u8) -> String { 
    let mut data = String::with_capacity(cypher.len());
    for ch in cypher.chars() {
        data.push(char_shift(ch, -1*key));
    }
    data
}
fn encrypt_caesar(data: &str, shift: i8) -> String {
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
fn char_shift(ch: char, shift: i8) -> char {
    if shift == 0 || shift > 25 {
        return ch;
    }
    let mut res = ch;
    let ch_i8 = ch as i8;
    match ch_i8 {
        65..=90 => {
            if ch_i8 + shift > 90 {
                res = (ch_i8 + shift - 90 + 64) as char;
            } else {
                res = (ch_i8 + shift) as char;
            }
        }
        97..=122 => {
            if ch_i8 + shift > 122 {
                res = (ch_i8 + shift - 122 + 96) as char;
            } else {
                res = (ch_i8 + shift) as char;
            }
        }
        _ => { /* return the same character that it receieved */ }
    }
    res
}
