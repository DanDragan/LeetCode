pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let len = nums.len();

    if len < 3 {
        return results;
    }

    let mut nums = nums;
    nums.sort_unstable();

    for i in 0..len - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let target = -nums[i];
        let mut left = i + 1;
        let mut right = len - 1;

        while left < right {
            let sum = nums[left] + nums[right];

            if sum == target {
                results.push(vec![nums[i], nums[left], nums[right]]);

                // Skip duplicate elements
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                left += 1;

                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    results
}

#[allow(dead_code)]
fn main () {

}

#[test]
fn test_two_sum() {
    assert!(three_sum(vec![-1,0,1,2,-1,-4]) == vec![vec![-1,-1,2], vec![-1,0,1]]);
}