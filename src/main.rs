fn main() {
    let matrix: [[i32; 6]; 6] = [
        [-9, -71, 44, 98, 21, 41],
        [50, -90, 10, 24, 53, 42],
        [9, 25, 35, 1, 2, 3],
        [4, 5, 8, 6, 19, 29],
        [47, 29, 49, 69, 35, 39],
        [52, 53, 54, 56, 66, -1000],
    ];

    let subset_size = 3; // size of the subset (3x3 matrix)

    let mut max_sum = std::i32::MAX;
    let mut max_subset = [[0; 3]; 3];

    for i in 0..=matrix.len() - subset_size {
        for j in 0..=matrix[i].len() - subset_size {
            let mut subset_sum = 0;
            let mut subset = [[0; 3]; 3];

            for row in 0..subset_size {
                for col in 0..subset_size {
                    subset_sum += matrix[i + row][j + col];
                    subset[row][col] = matrix[i + row][j + col];
                }
            }

            subset_sum -= matrix[i + 1][j] + matrix[i + 1][j + 2];

            if subset_sum < max_sum {
                max_sum = subset_sum;
                max_subset = subset;
            }
        }
    }

    println!("Maximum Subset:");
    for row in &max_subset {
        println!("{:?}", row);
    }

    println!("Maximum Subset Sum: {}", max_sum);
}
