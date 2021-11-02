#![deny(clippy::pedantic)]
mod caesar;
mod vigenere;
use std::{collections::HashMap, env};
use rand::{thread_rng, Rng};
fn main() {
    test_caesar();
    test_vigenere();
}

// A vigenere cipher is different in that you provide it a key, and based on the
// numerical position of the key, you apply a shift
// Todo: Test passing optional keys to this function. If not use a default like "duh"
fn test_vigenere() {
    let names = vec![
        "Leonardo",
        "Schrodinger",
        "Heisenberg",
        "Einstein",
        "Pauli",
        "Neumann",
    ];
    let key = "duh";

    let mut encrypted_names: Vec<String> = vec![];
    for &name in &names {
        encrypted_names.push(vigenere::encrypt(&name, &key));
    }

    let mut decrypted_names: Vec<String> = Vec::with_capacity(encrypted_names.len());
    for cipher in encrypted_names.iter() {
        decrypted_names.push(vigenere::decrypt(cipher, &key));
    }
    assert_eq!(&decrypted_names, &names);
    // println!("Encrypted names: {:?}", encrypted_names);
    // println!("Decrypted names: {:?}", decrypted_names);
}

fn test_caesar() {
    // A caesar cipher simply all letters of a message by a fixed amount
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
    // Test for all possible shift combinations, terminate only when all values have been tested
    loop {
        if shift_map.values().all(|shift_val| *shift_val == true) {
            break;
        }
        let shift = rng.gen_range(1..=25);
        // println!("Caesar shift value generated: {}", shift);
        shift_map.insert(shift, true);

        // Note the `&` before `names`. That is intentional because a for..loop consmues iterators by
        // calling into_iter() on our iterable. To prevent that, since we are using nested loops, we
        // use reference values
        for &name in &names {
            let encrypted = caesar::encrypt(name, shift);
            // println!("{} encrypted to {}", name, encrypted);
            assert_eq!(name, caesar::decrypt(&encrypted, shift));
        }
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
                    if ch_i8.overflowing_add(shift).1 || ch_i8.overflowing_add(shift).0 > 90 {
                        res = (ch_i8 + (shift - 90 + 64)) as u8 as char;
                    } else {
                        res = (ch_i8 + shift) as u8 as char;
                    }
                }
                97..=122 => {
                    if ch_i8.overflowing_add(shift).1 || ch_i8.overflowing_add(shift).0 > 122 {
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
