pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn caesar(input: &str, shift: i32) -> String {
    let mut result = String::new();
    for ch in input.chars() {
        if ch.is_ascii_lowercase() {
            // Find position in alphabet, apply shift with wrapping, push new char
            let pos = (ch as i32 - 'a' as i32) as u32;
            let new_pos = (pos as i32 + shift).rem_euclid(ALPHABET.len() as i32) as usize;
            result.push(ALPHABET.chars().nth(new_pos).unwrap());
        } else if ch.is_ascii_uppercase() {
            // Shift uppercase letter
            let pos = (ch as i32 - 'A' as i32) as u32;
            let new_pos = (pos as i32 + shift).rem_euclid(ALPHABET.len() as i32) as usize;
            let lowercase_char = ALPHABET.chars().nth(new_pos).unwrap();
            result.push(lowercase_char.to_uppercase().next().unwrap());
        } else {
            // Non-letters: keep unchanged
            result.push(ch);
        }

    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_shift_three() {
        assert_eq!(caesar("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn shift_minus_one() {
        assert_eq!(caesar("abc", -1), "zab");
    }

    #[test]
    fn shift_twenty_seven_wraps_to_one() {
        assert_eq!(caesar("xyz", 27), "yza");
    }

    #[test]
    fn empty_input() {
        assert_eq!(caesar("", 5), "");
    }

    #[test]
    fn shift_zero_is_identity() {
        assert_eq!(caesar("Rust!", 0), "Rust!");
    }

    #[test]
    fn shift_twenty_six_is_identity() {
        assert_eq!(caesar("abc", 26), "abc");
    }

    #[test]
    fn non_letters_preserved() {
        assert_eq!(caesar("1 2 3 !", 5), "1 2 3 !");
    }

    #[test]
    fn large_negative_shift_wraps() {
        assert_eq!(caesar("abc", -27), "zab");
    }
}
