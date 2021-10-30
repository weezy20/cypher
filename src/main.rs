#![allow(unused, dead_code)]
// Testing symmetric ciphers
// Caesar and Vigenere cypher
use std::fmt::{Debug, Display};
use rand::prelude::*;
fn main() {
    // let a = 'a';
    // print(a as usize);
    // print_char(127);
    // print_char(139);
    // a = 3 = 0011
    // a >> 1 = 0001
    // should print 1
}

struct Cypher {
    data: String,
    secret_key: u8,
    cipher: Option<String>,
}
impl Cypher {
    fn new(data: &str) -> Self {
        let mut rng = thread_rng();
        let data = String::from(data);
        let secret_key = rng.gen_range(1..=25);
        Self {
            data,
            secret_key,
            cipher: None,
        }
    }
    fn caesar_cipher(self) -> Self {
        let mut cipher = String::with_capacity(self.data.len());
        for (i, c) in self.data.char_indices() {
            
            let c_u8 = c as u8;
            let shift = c_u8 + self.secret_key;
            

        }
        Self {
            data: self.data,
            secret_key: self.secret_key,
            cipher: Some(cipher)
        }
    }
}

fn print_char(a: usize) {
    let a_u8 = a as u8;
    if a_u8 > 127 {
        println!("Not printable");
        return;
    }
    println!("printing {} as char --> {}", a, a_u8 as char);
}

fn print<T: Display>(a: T) {
    println!("printing -> {}", a);
}
