pub fn reverse(x: i32) -> i32 {
    let mut rev : i32 = 0;
    let mut num = x;
    let mul;

    if num < 0 {
        mul = -1;
        num *= -1;
    }
    else {
        mul = 1;
    }

    while num > 0 {
        if (std::i32::MAX - (num % 10)) / 10 >= rev {
            rev = rev * 10 + (num % 10);
            num = num / 10;
        }
        else {
            rev = 0;
            break;
        }
    }

    rev * mul
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_reverse() {
    assert!(reverse(123) == 321);

    assert!(reverse(-123) == -321);

    assert!(reverse(120) == 21);

    assert!(reverse(-120) == -21);

    assert!(reverse(1534236469) == 0);

    assert!(reverse(1463847412) == 2147483641);
}

