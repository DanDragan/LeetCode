use std::collections::HashMap;

pub fn int_to_roman(num: i32) -> String {
    let mut integer = num;
    let mut roman = String::new();
    let mut count = 4;

    let map = HashMap::from([
        (0, vec!['I', 'V', 'X']),
        (1, vec!['X', 'L', 'C']),
        (2, vec!['C', 'D', 'M']),
        (3, vec!['M', '-', '-']),
    ]);

    while integer > 0 {
        count -= 1;
        let quotient = integer / 10_i32.pow(count);
        integer = integer % 10_i32.pow(count);

        if quotient > 0 {
            if quotient <= 3 {
                for _ in 0..quotient {
                    roman.push(map[&count][0]);
                }
            }
            else if quotient == 4 {
                roman.push(map[&count][0]);
                roman.push(map[&count][1]);
            }
            else if quotient == 5 {
                roman.push(map[&count][1]);
            }
            else if quotient >= 6 && quotient <= 8 {
                roman.push(map[&count][1]);
                for _ in 0..quotient - 5 {
                    roman.push(map[&count][0]);
                }
            }
            else {
                roman.push(map[&count][0]);
                roman.push(map[&count][2]);
            }
        }
    }

    roman
}

#[allow(dead_code)]
fn main () {

}

#[test]
fn test_int_to_roman() {
    assert!(int_to_roman(3000) == "MMM".to_string());
    assert!(int_to_roman(1274) == "MCCLXXIV".to_string());
    assert!(int_to_roman(1944) == "MCMXLIV".to_string());
    assert!(int_to_roman(19) == "XIX".to_string());
    assert!(int_to_roman(17) == "XVII".to_string());
}
