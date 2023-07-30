pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let nums_a : Vec<i32>;
    let nums_b : Vec<i32>;

    if nums1.len() > nums2.len() {
        nums_a = nums2;
        nums_b = nums1;
    }
    else {
        nums_a = nums1;
        nums_b = nums2;
    }

    let m = nums_a.len();
    let n = nums_b.len();
    let mut left = 0;
    let mut right = m;
        
    while left <= right {
        let part_a = (left + right) / 2;
        let part_b = (m + n + 1) / 2 - part_a;
        
        let max_left_a = if part_a == 0  {std::i32::MIN} else {nums_a[part_a - 1]};
        let min_right_a = if part_a == m {std::i32::MAX} else {nums_a[part_a]};
        let max_left_b = if part_b == 0  {std::i32::MIN} else {nums_b[part_b - 1]};
        let min_right_b = if part_b == n {std::i32::MAX} else {nums_b[part_b]};
        
        if max_left_a <= min_right_b && max_left_b <= min_right_a {
            if (m + n) % 2 == 0 {
                return ((std::cmp::max(max_left_a, max_left_b) as f64) + (std::cmp::min(min_right_a, min_right_b) as f64)) / 2.0;
            } else {
                return std::cmp::max(max_left_a, max_left_b) as f64;
            }
        } else if max_left_a > min_right_b {
            right = part_a - 1;
        } else {
            left = part_a + 1;
        }
    }
    
    return 0.0;
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_reverse() {
    let nums1 = vec![1,2];
    let nums2 = vec![3,4];
    assert!((find_median_sorted_arrays(nums1, nums2) - 2.5).abs() < 0.0001);

    let nums3 = vec![3,5,8,15,32,35,41,45];
    let nums4 = vec![7,9,12,18,27,29,31,43];

    assert!((find_median_sorted_arrays(nums3, nums4) - 22.5).abs() < 0.0001);
}
