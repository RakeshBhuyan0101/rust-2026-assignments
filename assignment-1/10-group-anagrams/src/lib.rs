pub fn group_anagrams(words: &[String]) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    
    for word in words {
        // Create signature: sort lowercased characters
        let mut signature: Vec<char> = word.to_lowercase().chars().collect();
        signature.sort();
        let sig_string: String = signature.into_iter().collect();
        
        // Group by signature, preserving input order
        map.entry(sig_string)
            .or_insert_with(Vec::new)
            .push(word.clone());
    }
    
    // Convert HashMap to Vec of groups
    map.into_iter().map(|(_, group)| group).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(xs: &[&str]) -> Vec<String> {
        xs.iter().map(|x| x.to_string()).collect()
    }

    fn sort_groups(mut v: Vec<Vec<String>>) -> Vec<Vec<String>> {
        v.sort_by(|a, b| {
            let ak = a.first().map(|s| s.as_str()).unwrap_or("");
            let bk = b.first().map(|s| s.as_str()).unwrap_or("");
            ak.cmp(bk)
        });
        v
    }

    #[test]
    fn classic_example() {
        let input = s(&["Eat", "tea", "tan", "ate", "Nat", "bat"]);
        let expected = vec![
            s(&["Eat", "tea", "ate"]),
            s(&["bat"]),
            s(&["tan", "Nat"]),
        ];
        assert_eq!(sort_groups(group_anagrams(&input)), expected);
    }

    #[test]
    fn empty_input() {
        let input: Vec<String> = vec![];
        let expected: Vec<Vec<String>> = vec![];
        assert_eq!(group_anagrams(&input), expected);
    }

    #[test]
    fn no_anagrams() {
        let input = s(&["abc", "def"]);
        let expected = vec![s(&["abc"]), s(&["def"])];
        assert_eq!(sort_groups(group_anagrams(&input)), expected);
    }

    #[test]
    fn all_anagrams() {
        let input = s(&["abc", "bca", "cab"]);
        let expected = vec![s(&["abc", "bca", "cab"])];
        assert_eq!(sort_groups(group_anagrams(&input)), expected);
    }

    #[test]
    fn single_word() {
        let input = s(&["hello"]);
        let expected = vec![s(&["hello"])];
        assert_eq!(sort_groups(group_anagrams(&input)), expected);
    }

    #[test]
    fn case_insensitive_listen_silent() {
        let input = s(&["Listen", "Silent"]);
        let expected = vec![s(&["Listen", "Silent"])];
        assert_eq!(sort_groups(group_anagrams(&input)), expected);
    }

    #[test]
    fn preserves_input_order_within_group() {
        let input = s(&["abc", "cab", "bca"]);
        let groups = group_anagrams(&input);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0], s(&["abc", "cab", "bca"]));
    }

    #[test]
    fn different_lengths_dont_merge() {
        let input = s(&["a", "ab", "ba"]);
        let expected = vec![s(&["a"]), s(&["ab", "ba"])];
        assert_eq!(sort_groups(group_anagrams(&input)), expected);
    }
}
