pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let len = digits.len();
    let mut new_len = len + 1;
    let mut carry = 1;
    let mut j = len - 1;

    for i in 0..len {
        if digits[i] != 9 {
            new_len = len;
            break;
        }
    }

    let mut summed_digits = Vec::<i32>::new();
    summed_digits.resize(new_len, 0);

    for i in (0..=new_len-1).rev() {
        let sum = digits[j] + carry;

        summed_digits[i] = sum % 10;
        carry = sum / 10;

        if j > 0 {
            j -= 1;
        }
    }

    if new_len != len {
        summed_digits[0] = carry;
    }

    summed_digits
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_plus_one() {
    assert!(plus_one(vec![1,2,3]) == vec![1,2,4]);

    assert!(plus_one(vec![4,3,2,1]) == vec![4,3,2,2]);

    assert!(plus_one(vec![9]) == vec![1,0]);

    assert!(plus_one(vec![8,9,9,9]) == vec![9,0,0,0]);

    assert!(plus_one(vec![9,8,9]) == vec![9,9,0]);
}
