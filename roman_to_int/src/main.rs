pub fn roman_to_int(s: String) -> i32 {
    let mut number = 0;
    let mut i = 0;

    while i < s.len() {
        match s.chars().nth(i).unwrap() {
            'M' => number += 1000,

            'D' => number += 500,

            'C' => {
                if i != s.len() - 1 && s.chars().nth(i+1).unwrap() == 'D' {
                    number += 400;
                    i += 1;
                }
                else if i != s.len() - 1 && s.chars().nth(i+1).unwrap() == 'M' {
                    number += 900;
                    i += 1;
                }
                else {
                    number += 100;
                }
            },

            'L' => number += 50,

            'X' => {
                if i != s.len() - 1 && s.chars().nth(i+1).unwrap() == 'C' {
                    number += 90;
                    i += 1;
                }
                else if i != s.len() - 1 && s.chars().nth(i+1).unwrap() == 'L' {
                    number += 40;
                    i += 1;
                }
                else {
                    number += 10;
                }
            },

            'V' => number += 5,

            'I' => {
                if i != s.len() - 1 && s.chars().nth(i+1).unwrap() == 'V' {
                    number += 4;
                    i += 1;
                }
                else if i != s.len() - 1 && s.chars().nth(i+1).unwrap() == 'X' {
                    number += 9;
                    i += 1;
                }
                else {
                    number += 1;
                }
            },

            _ => println!("Incorrect roman literal"),
        }

        i += 1;
    }

    number
}

#[allow(dead_code)]
fn main () {

}

#[test]
fn test_roman_to_int() {
    assert!(roman_to_int("MCCLXXIV".to_string()) == 1274);
    assert!(roman_to_int("MCMXLIV".to_string()) == 1944);
    assert!(roman_to_int("XIX".to_string()) == 19);
    assert!(roman_to_int("XIVIII".to_string()) == 17);
}
