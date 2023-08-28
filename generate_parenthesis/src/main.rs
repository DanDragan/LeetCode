pub fn rec_generate(res: &mut Vec<String>, n: i32, opened: i32, closed: i32, str : &mut String) {
    if str.len() == (2 * n) as usize {
        res.push(str.to_string());
    }

    else {
        if opened < n {
            str.push_str("(");
            rec_generate(res, n, opened + 1, closed, str);
            str.pop();
        }

        if closed < opened {
            str.push_str(")");
            rec_generate(res, n, opened, closed + 1, str);
            str.pop();
        }
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let mut s = String::new();
    rec_generate(&mut res, n, 0, 0, &mut s);

    res
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_generate_parenthesis() {
    assert!(generate_parenthesis(3) == vec!["((()))","(()())","(())()","()(())","()()()"]);

    assert!(generate_parenthesis(1) == vec!["()"]);
}