use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("ERROR: could not read input");
    let mut num_xmas = 0;
    let num_cols = contents.find("\n").expect("");
    let num_lines = 1 + contents.match_indices("\n")
                    .collect::<Vec<_>>()
                    .len();
    let contents = contents.replace("\n", "");  

    // Construct a 2D Vector
    let mut field = Vec::new();
    for i in 0..num_lines {
        let mut line = Vec::new();
        for j in 0..num_cols {
            match contents.chars().nth(i*num_lines + j) {
                Some(x) => line.push(x),
                None => println!("This is a bug")
            }
        }
        field.push(line);
    }
    
    // Counting horizontal occurrences
    for i in 0..num_lines {
        let line = field[i].iter().collect::<String>();
        let reversed_line = line.chars().rev().collect::<String>();
        num_xmas += line.match_indices("XMAS").collect::<Vec<_>>().len();
        num_xmas += reversed_line.match_indices("XMAS").collect::<Vec<_>>().len();
    }
    
    for j in 0..num_cols {
        let mut column = vec![];
        for i in 0..num_lines {
            column.push(field[i][j]);
        }
        let column = column.iter().collect::<String>();
        let column_rev = column.chars().rev().collect::<String>();
        num_xmas += column.match_indices("XMAS").collect::<Vec<_>>().len();
        num_xmas += column_rev.match_indices("XMAS").collect::<Vec<_>>().len();
    }

    // Counting diagonal occurrences
    // 0 1 2 3 4 5
    // 0 0 0 0 0 0 0
    // 0 0 0 0 0 0 1
    // 0 0 0 0 0 0 2
    // 0 0 0 0 0 0 3
    // 0 0 0 0 0 0 4
    // 0 0 0 0 0 0 5
    for n in 0..num_lines+num_cols-1 {
        let mut diagonal = vec![];
        // i + j = n
        for i in 0..num_lines {
            let j: i32 = n as i32 - i as i32;
            if j >= 0 && j < num_cols as i32 { diagonal.push(field[i][j as usize]) };
        }
        let diagonal = diagonal.iter().collect::<String>();
        let reversed_diagonal = diagonal.chars().rev().collect::<String>();
        num_xmas += diagonal.match_indices("XMAS").collect::<Vec<_>>().len();
        num_xmas += reversed_diagonal.match_indices("XMAS").collect::<Vec<_>>().len();
    }
    for k in 0..num_cols+num_lines-1 {
        let n: i32 = k as i32 - num_lines as i32 + 1;
        let mut diagonal = vec![];
        // i - j = n
        for i in 0..num_lines {
            let j: i32 = i as i32 - n as i32 + 1;
            if j >= 0 && j < num_cols as i32 { diagonal.push(field[i][j as usize]) };
        }
        let diagonal = diagonal.iter().collect::<String>();
        let reversed_diagonal = diagonal.chars().rev().collect::<String>();
        num_xmas += diagonal.match_indices("XMAS").collect::<Vec<_>>().len();
        num_xmas += reversed_diagonal.match_indices("XMAS").collect::<Vec<_>>().len();
    }

    println!("Number of XMAS occurrences = {num_xmas}");
}