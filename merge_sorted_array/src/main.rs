pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut last = m+n-1;
    let mut last_nums1 = m;
    let mut last_nums2 = n;

    while last_nums1 > 0 && last_nums2 > 0 {
        if nums2[(last_nums2 - 1) as usize] > nums1[(last_nums1 - 1) as usize] {
            nums1[last as usize] = nums2[(last_nums2 - 1) as usize];
            last_nums2 = last_nums2 - 1;
        }
        else {
            nums1[last as usize] = nums1[(last_nums1 - 1) as usize];
            last_nums1 = last_nums1 - 1;
        }

        last = last -1;
    }

    if last_nums1 == 0 && last_nums2 > 0 {
        for i in 0..last_nums2 as usize {
            nums1[i] = nums2[i];
        }
    }

    if last_nums2 == 0 && last_nums1 > 0 {
        for i in 0..last_nums1 as usize {
            nums1[i] = nums1[i];
        }
    }
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_merge() {
    let mut nums1 : Vec<i32> = vec![1,2,3,0,0,0];
    let mut nums2 : Vec<i32> = vec![2,5,6];
    merge(&mut nums1, 3, &mut nums2, 3);
    assert!(nums1 == vec![1,2,2,3,5,6]);

    let mut nums3 : Vec<i32> = vec![1];
    let mut nums4 : Vec<i32> = vec![];
    merge(&mut nums3, 1, &mut nums4, 0);
    assert!(nums3 == vec![1]);

    let mut nums5 : Vec<i32> = vec![0];
    let mut nums6 : Vec<i32> = vec![1];
    merge(&mut nums5, 0, &mut nums6, 1);
    assert!(nums5 == vec![1]);

    let mut nums7 : Vec<i32> = vec![2, 0];
    let mut nums8 : Vec<i32> = vec![1];
    merge(&mut nums7, 1, &mut nums8, 1);
    assert!(nums7 == vec![1, 2]);
}
