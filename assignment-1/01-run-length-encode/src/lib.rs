pub fn run_length_encode(input: &str) -> Vec<(char, u32)> {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some( current_char) = chars.next() {
        let mut count = 1;
        while let Some(&next_char) = chars.peek(){
            if current_char == next_char {
                count +=1;
                chars.next();
            }
            else {
                break;
            }
        }
        result.push((current_char , count));
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_aaabbc() {
        assert_eq!(
            run_length_encode("aaabbc"),
            vec![('a', 3), ('b', 2), ('c', 1)]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(run_length_encode(""), vec![]);
    }

    #[test]
    fn single_char() {
        assert_eq!(run_length_encode("x"), vec![('x', 1)]);
    }

    #[test]
    fn all_same() {
        assert_eq!(run_length_encode("aaaaa"), vec![('a', 5)]);
    }

    #[test]
    fn all_different() {
        assert_eq!(
            run_length_encode("abcd"),
            vec![('a', 1), ('b', 1), ('c', 1), ('d', 1)]
        );
    }

    #[test]
    fn alternating_runs() {
        assert_eq!(
            run_length_encode("aabbaa"),
            vec![('a', 2), ('b', 2), ('a', 2)]
        );
    }

    #[test]
    fn whitespace_counts_too() {
        assert_eq!(
            run_length_encode("aa  bb"),
            vec![('a', 2), (' ', 2), ('b', 2)]
        );
    }
}
