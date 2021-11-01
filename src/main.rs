#![deny(clippy::pedantic)]

use std::{collections::HashMap, env};

use rand::{thread_rng, Rng};
fn main() {
    test_caesar();
}

fn test_caesar() {
    if env::args().any(|x| x == "trace" || x == "t") {
        env::set_var("RUST_BACKTRACE", "1");
    }
    // assert_eq!('c', char_shift('z', 3));
    let names = vec!["Joseph", "Matthew", "Jesus", "Zorba", "Abhishek"];
    let mut rng = thread_rng();
    let mut shift_map: HashMap<i8, bool> = HashMap::new();

    for i in 1..=25 {
        // populate our map with all possible shift values
        shift_map.insert(i, false);
    }
    loop {
        if shift_map.values().all(|shift_val| *shift_val == true) {
            break;
        }
        let shift = rng.gen_range(1..=25);
        // println!("Caesar shift value generated: {}", shift);
        shift_map.insert(shift, true);

        for &name in &names {
            let encrypted = caesar::encrypt(name, shift);
            // println!("{} encrypted to {}", name, encrypted);
            assert_eq!(name, caesar::decrypt(&encrypted, shift));
        }
    }
}
mod caesar {
    use super::*;
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
}

fn char_shift(ch: char, shift: i8) -> char {
    if shift == 0 || shift > 25 {
        return ch;
    }
    let mut res = ch;
    let ch_i8 = ch as i8;
    match shift {
        // Positive Shift
        shift @ 0..=i8::MAX => {
            match ch_i8 {
                65..=90 => {
                    // Logical operators short circuit in Rust so we can write this:
                    if ch_i8.overflowing_add(shift).1
                        || ch_i8.overflowing_add(shift).0 > 90
                    {
                        res = (ch_i8 + (shift - 90 + 64)) as u8 as char;
                    } else {
                        res = (ch_i8 + shift) as u8 as char;
                    }
                }
                97..=122 => {
                    if ch_i8.overflowing_add(shift).1
                        || ch_i8.overflowing_add(shift).0 > 122
                    {
                        res = (ch_i8 + (shift - 122 + 96)) as u8 as char;
                    } else {
                        res = (ch_i8 + shift) as u8 as char;
                    }
                }
                _ => { /* return the same character that it receieved */ }
            }
        }
        // Negative Shift
        unshift @ i8::MIN..=-1 => {
            let unshift = -unshift;
            match ch_i8 {
                65..=90 => {
                    if ch_i8 - unshift < 65 {
                        let diff = 65 - (ch_i8 - unshift);
                        res = ((90 - diff) + 1) as u8 as char;
                    } else {
                        res = (ch_i8 - unshift) as u8 as char;
                    }
                }
                97..=122 => {
                    if ch_i8 - unshift < 97 {
                        let diff = 97 - (ch_i8 - unshift);
                        res = ((122 - diff) + 1) as u8 as char;
                    } else {
                        res = (ch_i8 - unshift) as u8 as char;
                    }
                }
                _ => { /* return the same character that it receieved */ }
            }
        }
    }
    res
}
