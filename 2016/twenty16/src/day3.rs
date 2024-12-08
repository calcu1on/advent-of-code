use crate::Solution;
use std::fs;

pub fn day_3() -> Solution {
    let triangles_file = fs::read_to_string("./sources/day3.txt").expect("unable to read file");
    let triangles = triangles_file.split("\n");
    let mut valid_triangles: i32 = 0;
    let mut column_1: Vec<i32> = vec![];
    let mut column_2: Vec<i32> = vec![];
    let mut column_3: Vec<i32> = vec![];
    
    // Part 1.
    for triangle in triangles {
        // Skip if empty line.
        if triangle.len() < 1 {
            continue;
        }
        let isolate_sides = triangle.split("  ");
        let mut valid_sides: Vec<i32> = Vec::new();
        for side in isolate_sides {
            // Skip if found empty side value.
            if side.len() < 1 {
                continue;
            } else {
                let side_value: i32 = side.trim().parse().unwrap();
                valid_sides.push(side_value); 
            }
        }
        let is_valid_triangle = check_if_valid_triangle(valid_sides.clone());
        if is_valid_triangle {
            valid_triangles += 1;
        }
        column_1.push(valid_sides[0]);
        column_2.push(valid_sides[1]);
        column_3.push(valid_sides[2]);
    }
    let p1_solution = valid_triangles.to_string();
    // Part 2.
    valid_triangles = 0;
    // slice each column into groups of 3.
    for vert_triangle in column_1.chunks(3) {
        let is_valid_triangle = check_if_valid_triangle(vert_triangle.to_vec());
        if is_valid_triangle {
            valid_triangles += 1;
        }
    }
    for vert_triangle in column_2.chunks(3) {
       let is_valid_triangle = check_if_valid_triangle(vert_triangle.to_vec());
        if is_valid_triangle {
            valid_triangles += 1;
        }
    }
    for vert_triangle in column_3.chunks(3) {
        let is_valid_triangle = check_if_valid_triangle(vert_triangle.to_vec());
        if is_valid_triangle {
            valid_triangles += 1;
        }
    }
    let p2_solution = valid_triangles.to_string();

    let solution = Solution {
        day: 3,
        answer_1: p1_solution,
        answer_2: p2_solution
    };

    solution
}

fn check_if_valid_triangle(sides: Vec<i32>) -> bool {
    let side_a = sides[0];
    let side_b = sides[1];
    let side_c = sides[2];
    if side_a + side_b <= side_c || side_a + side_c <= side_b || side_b + side_c <= side_a {
        false
    } else {
        true
    }
    
}
