pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x;
    }
 
    let mut num = x;

    if x == std::i32::MAX {
        num = num - 1;
    }

    let mut y = num;
    let mut z = (y + (num / y)) / 2;
 
    while y > z {
        y = z;
        z = (y + (x / y)) / 2;
    }

    if z * z > x {
        z = z - 1;
    }

    z
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_reverse() {
    assert!(my_sqrt(8) == 2);
    assert!(my_sqrt(4) == 2);
    assert!(my_sqrt(9) == 3);
    assert!(my_sqrt(16) == 4);
    assert!(my_sqrt(35) == 5);
    assert!(my_sqrt(2147483647) == 46340);
}