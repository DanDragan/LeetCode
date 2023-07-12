pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut v = strs;
    v.sort_by(|a, b| a.len().cmp(&b.len()));

    let str = &v[0];
    let mut len = 0;

    for j in 1..str.len()+1 {
        let mut ok = true;
        for k in 0..v.len() {
            if !v[k][0..j].to_string().eq(&str[0..j].to_string()) {
                ok = false;
            }
        }

        if ok && j>len {
            len = j;
        }
    }

    str[0..len].to_string()
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_longest_common_prefix() {
    assert!(longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()])== "fl".to_string());
    assert!(longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()])== "".to_string());
    assert!(longest_common_prefix(vec!["cir".to_string(),"car".to_string()])== "c".to_string());
    assert!(longest_common_prefix(vec!["reflower".to_string(),"flow".to_string(),"flight".to_string()])== "".to_string());
    assert!(longest_common_prefix(vec!["a".to_string()])== "a".to_string());
}
