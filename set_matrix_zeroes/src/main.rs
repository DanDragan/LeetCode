pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut rows = false;
    let mut cols = false;
    let first = matrix[0][0];

    for j in 0..matrix[0].len() {
        if matrix[0][j] == 0 {
            cols = true;
        }
    }

    for i in 0..matrix.len() {
        if matrix[i][0] == 0 {
            rows = true;
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }

    for j in 1..matrix[0].len() {
        if matrix[0][j] == 0 {
            for i in 0..matrix.len() {
                matrix[i][j] = 0;
            }
        }
    }

    for i in 1..matrix.len() {
        if matrix[i][0] == 0 {
            for j in 0..matrix[i].len() {
                matrix[i][j] = 0;
            }
        }
    }

    if first == 0 {
        for i in 0..matrix.len() {
            matrix[i][0] = 0;
        }

        for j in 0..matrix[0].len() {
            matrix[0][j] = 0;
        }
    }
    
    if rows == true {
        for i in 0..matrix.len() {
            matrix[i][0] = 0;
        }
    }

    if cols == true {
        for j in 0..matrix[0].len() {
            matrix[0][j] = 0;
        }
    }
}

#[allow(dead_code)]
fn main() {
}

#[test]
fn test_set_zeroes() {
    let mut mat1 = vec![vec![1,1,1], vec![1,0,1], vec![1,1,1]];
    set_zeroes(&mut mat1);
    let res_mat1 = vec![vec![1,0,1], vec![0,0,0], vec![1,0,1]];
    for i in 0..mat1.len() {
        for j in 0..mat1[0].len() {
            assert!(mat1[i][j] == res_mat1[i][j]);
        }
    }

    let mut mat2 = vec![vec![0,1,2,0], vec![3,4,5,2], vec![1,3,1,5]];
    set_zeroes(&mut mat2);
    let res_mat2 = vec![vec![0,0,0,0], vec![0,4,5,0], vec![0,3,1,0]];
    for i in 0..mat2.len() {
        for j in 0..mat2[0].len() {
            assert!(mat2[i][j] == res_mat2[i][j]);
        }
    }

    let mut mat3 = vec![vec![1,2,3,4], vec![5,0,7,8], vec![0,10,11,12], vec![13,14,15,0]];
    set_zeroes(&mut mat3);
    let res_mat3 = vec![vec![0,0,3,0], vec![0,0,0,0], vec![0,0,0,0], vec![0,0,0,0]];
    for i in 0..mat3.len() {
        for j in 0..mat3[0].len() {
            assert!(mat3[i][j] == res_mat3[i][j]);
        }
    }

    let mut mat4 = vec![vec![1,0,3]];
    set_zeroes(&mut mat4);
    let res_mat4 = vec![vec![0,0,0]];
    for i in 0..mat4.len() {
        for j in 0..mat4[0].len() {
            assert!(mat4[i][j] == res_mat4[i][j]);
        }
    }
    
    let mut mat5 = vec![vec![-4,-2147483648,6,-7,0], vec![-8,6,-8,-6,0], vec![2147483647,2,-9,-6,-10]];
    set_zeroes(&mut mat5);
    let res_mat5 = vec![vec![0,0,0,0,0], vec![0,0,0,0,0], vec![2147483647,2,-9,-6,0]];
    for i in 0..mat5.len() {
        for j in 0..mat5[0].len() {
            assert!(mat5[i][j] == res_mat5[i][j]);
        }
    }

    let mut mat6 = vec![vec![8,3,6,9,7,8,0,6], vec![0,3,7,0,0,4,3,8], vec![5,3,6,7,1,6,2,6], vec![8,7,2,5,0,6,4,0], vec![0,2,9,9,3,9,7,3]];
    set_zeroes(&mut mat6);
    let res_mat6 = vec![vec![0,0,0,0,0,0,0,0], vec![0,0,0,0,0,0,0,0], vec![0,3,6,0,0,6,0,0], vec![0,0,0,0,0,0,0,0], vec![0,0,0,0,0,0,0,0]];
    for i in 0..mat6.len() {
        for j in 0..mat6[0].len() {
            assert!(mat6[i][j] == res_mat6[i][j]);
        }
    }
}
