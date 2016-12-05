use common;
use errors::*;

fn is_valid_triangle(triangle: &Vec<u32>) -> bool {
    let mut result = true;
    for side in 0..3 {
        let sum_of_others = (0..3).filter(|&i| i != side)
                                  .map(|i| triangle[i])
                                  .fold(0, |acc, length| acc+length);
        if sum_of_others <= triangle[side] {
            result = false;
        }
    }
    result
}

fn valid_triangles(triangles: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    triangles.iter()
             .filter(|t| is_valid_triangle(t))
             .map(|v| v.clone())
             .collect()
}

pub fn day3() -> Result<()> {
    let input = &mut String::new();
    common::read_file_to_string("input/day3.txt", input)?;

    // Read triangles as rows
    let row_triangles: Vec<Vec<u32>> =
        input.split("\n")
             .filter(|s| !s.is_empty())
             .map(|s| s.split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect())
             .collect();
    println!("Number of valid row triangles: {}",
             valid_triangles(&row_triangles).len());

    // Read triangles as columns :-/
    let mut num_valid_column_triangles = 0;
    for i in 0..row_triangles.len()/3 {
        let row = i * 3; // step_by() is unstable, so doing this instead.
        for col in 0..3 {
            let triangle = vec![row_triangles[row+0][col],
                                row_triangles[row+1][col],
                                row_triangles[row+2][col]];
            if is_valid_triangle(&triangle) {
                num_valid_column_triangles += 1;
            }
        }
    }

    println!("Number of valid col triangles: {}", num_valid_column_triangles);
    Ok(())
}
