pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area : usize = 0;
    let mut first : usize = 0;
    let mut last : usize = height.len() - 1;

    while first < last {
        let crt_area : usize = (std::cmp::min(height[first], height[last]) as usize) * (last - first);

        if max_area < crt_area {
            max_area = crt_area;
        }

        if height[first] < height[last] {
            first += 1;
        }
        else {
            last -= 1;
        }
    }

    max_area as i32
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_max_area() {
    assert!(max_area(vec![1,8,6,2,5,4,8,3,7]) == 49);

    assert!(max_area(vec![1,1]) == 1);
}