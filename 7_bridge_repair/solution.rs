use std::fs;

fn part_1(contents: &String) {
    let mut total = 0;
    for line in contents.lines() {
        let equation: Vec<&str> = line.split(":").collect();
        assert_eq!(equation.len(), 2);
        let actual = equation[0].parse::<usize>().expect("");
        let numbers: Vec<usize> = equation[1].split_whitespace().map(|s| s.parse::<usize>().expect("")).collect();
        let num_ops = numbers.len() - 1;
        for i in 0..usize::pow(2, num_ops as u32) {
            let mut result = numbers[0];
            for j in 0..num_ops {
                result = match i & usize::rotate_left(1, j as u32) == 0 {
                    true => result + numbers[j+1],
                    false => result * numbers[j+1]
                }
            }
            if result == actual { 
                total += result;
                break;
            }
        }
    }
    println!("Total from correct equations = {total}");
}

fn concatenate(x: usize, y: usize) -> usize {
    // This is a dumb way but bear with me here
    let mut _x = x.to_string();
    _x.push_str(&y.to_string());
    return _x.parse().expect("")
}

fn part_2(contents: &String) {
    let mut total = 0;
    for line in contents.lines() {
        let equation: Vec<&str> = line.split(":").collect();
        assert_eq!(equation.len(), 2);
        let actual = equation[0].parse::<usize>().expect("");
        let numbers: Vec<usize> = equation[1].split_whitespace().map(|s| s.parse::<usize>().expect("")).collect();
        let num_ops = numbers.len() - 1;
        for i in 0..usize::pow(3, num_ops as u32) {
            let mut result = numbers[0];
            for j in 0..num_ops {
                result = match (i / usize::pow(3, j as u32)) % 3  {
                    0 => result + numbers[j+1],
                    1 => result * numbers[j+1],
                    2 => concatenate(result, numbers[j+1]),
                    _ => panic!("Invalid result for modulo 3")
                }
            }
            if result == actual { 
                total += result;
                break;
            }
        }
    }
    println!("Total from correct equations = {total}");
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("");
    part_1(&contents);
    part_2(&contents);
}