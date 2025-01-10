use std::fs;

fn part_1(field: &Vec<Vec<char>>, num_lines: usize, num_cols: usize) {
    // NOTE: while this solution works, a more elegant solution
    // would be to identify all occurrences of 'X' and then
    // iterate 3 chars on each of the 8 directions, akin to part_2  
    let mut num_xmas = 0;
    
    // Counting horizontal occurrences
    for i in 0..num_lines {
        let line = field[i].iter().collect::<String>();
        let reversed_line = line.chars().rev().collect::<String>();
        num_xmas += line.match_indices("XMAS").collect::<Vec<_>>().len();
        num_xmas += reversed_line.match_indices("XMAS").collect::<Vec<_>>().len();
    }
    
    // Counting vertical occurrences
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

fn part_2(field: &Vec<Vec<char>>, num_lines: usize, num_cols: usize) {
    let mut num_xmas = 0;
    for i in 1..num_lines-1 {
        for j in 1..num_cols-1 {
            if field[i][j] == 'A' {
                let mut num_m = 0;
                let mut num_s = 0;
                if field[i-1][j-1] == 'M' { num_m += 1 }
                if field[i-1][j-1] == 'S' { num_s += 1 }
                if field[i+1][j-1] == 'M' { num_m += 1 }
                if field[i+1][j-1] == 'S' { num_s += 1 }
                if field[i-1][j+1] == 'M' { num_m += 1 }
                if field[i-1][j+1] == 'S' { num_s += 1 }
                if field[i+1][j+1] == 'M' { num_m += 1 }
                if field[i+1][j+1] == 'S' { num_s += 1 }
                if num_m != 2 || num_s != 2 { continue }
                if field[i+1][j+1] == 'S' && field[i-1][j-1] == 'S' { continue }
                if field[i+1][j+1] == 'M' && field[i-1][j-1] == 'M' { continue }
                num_xmas += 1;
            }
        }
    }
    println!("Number of X-MAS occurrences = {num_xmas}");
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("ERROR: could not read input");
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

    part_1(&field, num_lines, num_cols);
    part_2(&field, num_lines, num_cols);
}