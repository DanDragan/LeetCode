use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut v = Vec::<i32>::new();
    
    for i in 0..nums.len() {
        map.insert(target - nums[i], i);
    }

    for i in 0..nums.len() {
        if map.contains_key(&nums[i]) && map[&nums[i]] != i {
            v.push(i as i32);
            v.push(map[&nums[i]] as i32);
            break;
        }
    }

    v
}

#[allow(dead_code)]
fn main () {

}

#[test]
fn test_two_sum() {
    assert!(two_sum(vec![2,7,11,15], 9) == vec![0, 1]);
    assert!(two_sum(vec![3,2,4], 6) == vec![1, 2]);
    assert!(two_sum(vec![3,3], 6) == vec![0, 1]);
}