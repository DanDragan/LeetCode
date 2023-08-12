pub fn my_atoi(s: String) -> i32 {
    let mut num = 0;
    let char_vec: Vec<char> = s.chars().collect();

    let mut start = 0;
    let mut sign = 1;
    let mut overflow = false;

    if s == "" {
        return 0;
    }

    while start < char_vec.len() && char_vec[start] == ' ' {
        start += 1;
    }

    if start < char_vec.len() && char_vec[start] == '-' {
        sign = -1;
        start += 1;
    }
    else if start < char_vec.len() && char_vec[start] == '+' {
        start += 1;
    }

    for i in start..char_vec.len() {
        if char_vec[i].is_digit(10) {
            let digit = (char_vec[i] as i32) - 48;

            if (std::i32::MAX - digit) / 10 >= num {
                num = num * 10 + digit;
            }
            else {
                num = std::i32::MAX;
                overflow = true;
            }
        }
        else {
            break;
        }
    }

    if overflow && sign == -1 {
        return std::i32::MIN;
    }

    num * sign
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_my_atoi() {
    assert!(my_atoi("42".to_string()) == 42);

    assert!(my_atoi("   -42".to_string()) == -42);

    assert!(my_atoi("4193 with words".to_string()) == 4193);

    assert!(my_atoi("-91283472332".to_string()) == -2147483648);

    assert!(my_atoi("".to_string()) == 0);

    assert!(my_atoi("+1".to_string()) == 1);

    assert!(my_atoi(" ".to_string()) == 0);

    assert!(my_atoi("-2147483647".to_string()) == -2147483647);
}
