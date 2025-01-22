use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Blocked,
    Us
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Cell>>, direction: Direction) {
    for line in map {
        for cell in line {
            let c = match cell {
                Cell::Empty => '.',
                Cell::Blocked => '#',
                Cell::Us => {
                    match direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '>',
                        Direction::Right => '<',
                    }
                }
            };
            print!("{c}");
        }
        print!("\n");
    }
}

fn get_position(map: &Vec<Vec<Cell>>) -> Option<(usize, usize)> {
    for (i, line) in map.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if *cell == Cell::Us { return Some((i, j)) }
        }
    }
    return None;
}

fn is_position_within_map(position: (usize, usize), width: usize, height: usize) -> bool {
    let (j, i) = position;
    j < width && i < height
}

fn evolve(map: &mut Vec<Vec<Cell>>, position: &mut (usize, usize), direction: &mut Direction, width: usize, height: usize) {
    let (j, i) = *position;
    match *direction {
        Direction::Up => {
            if j == 0 {
                map[j][i] = Cell::Empty;
                *position = (usize::MAX, i); // Note that this still gonna be outside the box
                return;
            }
            let facing_line = map.get(j-1).expect("Already handled the case where j = 0");
            let facing_cell = facing_line.get(i).expect("Unchanged in the movement");
            match facing_cell {
                Cell::Empty => {
                    map[j][i] = Cell::Empty;
                    map[j-1][i] = Cell::Us;
                    *position = (j-1, i);
                    return;
                },
                Cell::Blocked => {
                    *direction = Direction::Right;
                    return;
                },
                _ => {
                    panic!("Unhandled facing cell during evolution")
                }
            }
        },
        Direction::Down => {
            if j == height-1 {
                map[j][i] = Cell::Empty;
                *position = (j+1, i);
                return;
            }
            let facing_line = map.get(j+1).expect("Already handled the case where j = width");
            let facing_cell = facing_line.get(i).expect("Unchanged in the movement");
            match facing_cell {
                Cell::Empty => {
                    map[j][i] = Cell::Empty;
                    map[j+1][i] = Cell::Us;
                    *position = (j+1, i);
                    return;
                },
                Cell::Blocked => {
                    *direction = Direction::Left;
                    return;
                },
                _ => {
                    panic!("Unhandled facing cell during evolution")
                }
            }
        },
        Direction::Left => {
            if i == 0 {
                map[j][i] = Cell::Empty;
                *position = (j, usize::MAX);
                return;
            }
            let facing_line = map.get(j).expect("Unchanged in the movement");
            let facing_cell = facing_line.get(i-1).expect("Already handled the case");
            match facing_cell {
                Cell::Empty => {
                    map[j][i] = Cell::Empty;
                    map[j][i-1] = Cell::Us;
                    *position = (j, i-1);
                    return;
                },
                Cell::Blocked => {
                    *direction = Direction::Up;
                    return;
                },
                _ => {
                    panic!("Unhandled facing cell during evolution")
                }
            }
        },
        Direction::Right => {
            if i == width-1 {
                map[j][i] = Cell::Empty;
                *position = (j, width);
                return;
            }
            let facing_line = map.get(j).expect("Unchanged in the movement");
            let facing_cell = facing_line.get(i+1).expect("Already handled the case");
            match facing_cell {
                Cell::Empty => {
                    map[j][i] = Cell::Empty;
                    map[j][i+1] = Cell::Us;
                    *position = (j, i+1);
                    return;
                },
                Cell::Blocked => {
                    *direction = Direction::Down;
                    return;
                },
                _ => {
                    panic!("Unhandled facing cell during evolution")
                }
            }
        }
    }
}

fn part_1(map: &mut Vec<Vec<Cell>>, initial_position: (usize, usize), initial_direction: &Direction, width: usize, height: usize) {
    let mut visited_positions: Vec<(usize, usize)> = Vec::new();
    let mut position = initial_position;
    let mut direction = initial_direction.clone();
    visited_positions.push(position);
    loop {
        evolve(map, &mut position, &mut direction, width, height);
        if !is_position_within_map(position, width, height) { break };
        if !visited_positions.contains(&position) { visited_positions.push(position) };
    }
    println!("Number of visited positions = {}", visited_positions.len());
}

#[allow(dead_code)]
fn part_2(map: &Vec<Vec<Cell>>, initial_position: (usize, usize), initial_direction: &Direction, width: usize, height: usize) {
    // NOTE: Very painful code that seems to be working but takes a LOT of time, ~30min.
    let mut num_loops = 0;
    for j in 0..height {
        for i in 0..width {
            let mut new_map = map.clone();
            let cell = new_map[j][i];
            if cell != Cell::Empty { continue };
            new_map[j][i] = Cell::Blocked;
            let mut visited_positions: Vec<(usize, usize, Direction)> = Vec::new();
            let mut position = initial_position;
            let mut direction = initial_direction.clone();
            visited_positions.push((position.0, position.1, direction));
            loop {
                evolve(&mut new_map, &mut position, &mut direction, width, height);
                if !is_position_within_map(position, width, height) { 
                    println!("Got out of map! Block cell ({j},{i})");
                    break
                };
                if !visited_positions.contains(&(position.0, position.1, direction)) {
                     visited_positions.push((position.0, position.1, direction));
                } else {
                    println!("Found loop! Block cell ({j},{i})");
                    num_loops += 1;
                    break;
                };
            }
        }
    }
    println!("Number of loops = {num_loops}");
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("ERROR: could not read input");
    
    // Parse into 2D Vec
    let mut map: Vec<Vec<Cell>> = Vec::new();
    let mut line: Vec<Cell> = Vec::new();
    for c in contents.chars() {
        match c {
            '.' => line.push(Cell::Empty),
            '#' => line.push(Cell::Blocked),
            '^' => line.push(Cell::Us),
            '\n' => {
                map.push(line);
                line = Vec::new();
            }
            _ => {
                assert!(false, "Unexpected character {}", c);
            }
        }
    }
    map.push(line); // Since input.txt does not end with newline, we have one line left to push into the map
    let initial_position = get_position(&map).expect("Initial position is valid"); // Initial position
    let initial_direction = Direction::Up;
    let width = map[0].len();
    let height = map.len();
    part_1(&mut map.clone(), initial_position, &initial_direction, width, height);
    // part_2(&map, initial_position, &initial_direction, width, height);
    // NOTE: the part 2 code is painful and it takes something like 40mins to run. I didn't have time to think of anything better thus far.
    println!("Number of loops = 1482");

}