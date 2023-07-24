pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ivals = intervals;
    ivals.sort_by_key(|x| x[0]);

    let mut res_ivals = vec![vec![ivals[0][0], ivals[0][1]]];
    let mut crt = 0;

    for i in 1..ivals.len() {
        if res_ivals[crt][1] >= ivals[i][0] && res_ivals[crt][1] <= ivals[i][1] {
            res_ivals[crt][1] = ivals[i][1];
        }
        else if res_ivals[crt][1] > ivals[i][1] {
            continue;
        }
        else {
            res_ivals.push(vec![ivals[i][0], ivals[i][1]]);
            crt = crt + 1;
        }
    }
    res_ivals
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_merge_interval() {
    let intervals1 = vec![vec![1,3], vec![2,6], vec![8,10], vec![15,18]];
    assert!(merge(intervals1) == vec![vec![1,6], vec![8,10], vec![15,18]]);

    let intervals2 = vec![vec![1,4], vec![4,5]];
    assert!(merge(intervals2) == vec![vec![1,5]]);

    let intervals2 = vec![vec![1,4], vec![2,3]];
    assert!(merge(intervals2) == vec![vec![1,4]]);
}
