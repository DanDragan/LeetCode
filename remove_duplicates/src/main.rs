pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 0;

    let mut unique_nums = vec![0; 201];

    for i in 0..nums.len() {
        unique_nums[(nums[i] + 100) as usize] = 1;
    }

    for i in 0..unique_nums.len() {
        if unique_nums[i] == 1 {
            nums[k] = (i as i32) - 100;
            k = k + 1;
        }
    }

    k as i32
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_reverse() {
    let mut v1 = vec![1,1,2];
    let k1 = 2;
    assert!(remove_duplicates(&mut v1) == k1);

    let v1_res = vec![1,2];

    for i in 0..k1 as usize {
        assert!(v1[i] == v1_res[i]);
    }

    let mut v2 = vec![0,0,1,1,1,2,2,3,3,4];
    let k2 = 5;
    assert!(remove_duplicates(&mut v2) == k2);

    let v2_res = vec![0,1,2,3,4];

    for i in 0..k2 as usize {
        assert!(v2[i] == v2_res[i]);
    }
}

