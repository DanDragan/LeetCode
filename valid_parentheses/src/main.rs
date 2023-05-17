pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::<char>::new();
    let mut valid = true;

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                stack.push(c);
            },

            ')' => {
                if stack.len() == 0 || stack.pop() != Some('(') {
                    valid = false;
                }
            },

            ']' => {
                if stack.len() == 0 || stack.pop() != Some('[') {
                    valid = false;
                }
            },

            '}' => {
                if stack.len() == 0 || stack.pop() != Some('{') {
                    valid = false;
                }
            },

            _ => println!("There should be no other character!"),
        }
    }

    if stack.len() != 0 {
        valid = false;
    }

    valid
}

#[allow(dead_code)]
fn main () {

}

#[test]
fn test_is_valid() {
    assert!(is_valid("()".to_string()) == true);
    assert!(is_valid("()[]{}".to_string()) == true);
    assert!(is_valid("(]".to_string()) == false);
}