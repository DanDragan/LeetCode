pub fn str_str(haystack: String, needle: String) -> i32 {
    let h_chars : Vec<char> = haystack.chars().collect();
    let n_chars : Vec<char> = needle.chars().collect();

    if h_chars.len() < n_chars.len() {
        return -1;
    }

    for i in 0..=h_chars.len() - n_chars.len() {
        if n_chars[0] == h_chars[i] {
            let mut ok = true;
            for j in 0..n_chars.len() {
                if n_chars[j] != h_chars[i+j] {
                    ok = false;
                    break;
                }
            }

            if ok {
                return i as i32;
            }
        }
    }

    return -1;
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_str_str() {
    assert!(str_str("sadbutsad".to_string(), "sad".to_string()) == 0);

    assert!(str_str("leetcode".to_string(), "leeto".to_string()) == -1);

    assert!(str_str("aaa".to_string(), "aaaa".to_string()) == -1);

    assert!(str_str("mississippi".to_string(), "issipi".to_string()) == -1);

    assert!(str_str("a".to_string(), "a".to_string()) == 0);
}
