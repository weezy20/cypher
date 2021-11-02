#![allow(unused)]
use super::char_shift;
pub(crate) fn encrypt(data: &str, key: &str) -> String {
    let mut keys = get_shift_vals(&key);
    let mut res = String::with_capacity(data.len());
    res = data
        .chars()
        .map(|ch| char_shift(ch, keys.next().unwrap() as i8))
        .collect();
    res
}
pub(crate) fn decrypt(data: &str, key: &str) -> String {
    // We cannot accept the previous keys as the cycle may have left it in an unsound state, so
    // we instantiate our own cycle here:
    let mut keys = get_shift_vals(&key);

    data.chars()
        .map(|ch| char_shift(ch, -(keys.next().unwrap() as i8)))
        .collect::<String>()
}
// Take a string key and return a cycle over shift values which are positions of individual letters
fn get_shift_vals(key: &str) -> impl Iterator<Item = u8> {
    key.to_lowercase()
        .chars()
        .map(|ch| ch as u8 - 97)
        .collect::<Vec<u8>>()
        .into_iter()
        .cycle()
}
