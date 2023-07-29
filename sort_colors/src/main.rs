pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut num_red = 0;
    let mut num_white = 0;
    let mut num_blue = 0;

    for i in 0..nums.len() {
        match nums[i] {
            0 => num_red = num_red + 1,

            1 => num_white = num_white + 1,

            2 => num_blue = num_blue + 1,
            
            _ => println!("Invalid color code"),
        }
    }

    for i in 0..num_red {
        nums[i] = 0;
    }

    for i in 0..num_white {
        nums[num_red + i] = 1;
    }

    for i in 0..num_blue {
        nums[num_red + num_white + i] = 2;
    }
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_sort_colors() {
    let mut nums1 = vec![2,0,2,1,1,0];
    sort_colors(&mut nums1);
    assert!(nums1 == vec![0,0,1,1,2,2]);

    let mut nums2 = vec![2,0,1];
    sort_colors(&mut nums2);
    assert!(nums2 == vec![0,1,2]);
}
