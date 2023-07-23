pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;
    let size = nums.len();
    let mut last = size - 1;

    for i in 0..nums.len() {
        while nums[last] == val && last > 0 {
            nums[last] = std::i32::MAX;
            last = last - 1;
            k = k + 1;
        }

        if nums[i] == val && last >= i {
            if last > 0 {
                nums[i] = nums[last];
                last = last - 1;
                k = k + 1;
            }
            else {
                if nums[last] == val {
                    k = k + 1;
                }
            }
        }
    }

    (size - k) as i32
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_remove_element() {
    let mut nums1 : Vec<i32> = vec![0,1,2,2,3,0,4,2];
    let val1 = 2;

    let k1 = remove_element(&mut nums1, val1);
    assert!(k1 == 5);

    for i in 0..k1 as usize {
        assert!(nums1[i] != val1);
    }

    let mut nums2 : Vec<i32> = vec![3,2,2,3];
    let val2 = 3;

    let k2 = remove_element(&mut nums2, val2);
    assert!(k2 == 2);

    for i in 0..k2 as usize {
        assert!(nums2[i] != val2);
    }

    let mut nums3 : Vec<i32> = vec![1];
    let val3 = 1;

    let k3 = remove_element(&mut nums3, val3);
    assert!(k3 == 0);

    for i in 0..k3 as usize {
        assert!(nums3[i] != val3);
    }

    let mut nums4 : Vec<i32> = vec![3,3];
    let val4 = 3;

    let k4 = remove_element(&mut nums4, val4);
    assert!(k4 == 0);

    for i in 0..k4 as usize {
        assert!(nums4[i] != val4);
    }

    let mut nums5 : Vec<i32> = vec![2,2,3];
    let val5 = 3;

    let k5 = remove_element(&mut nums5, val5);
    assert!(k5 == 2);

    for i in 0..k5 as usize {
        assert!(nums5[i] != val5);
    }
}
