use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum CellKind {
    Empty,
    Blocked,
    Us
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Cell {
    kind: CellKind,
    visited: bool
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

type Map = Vec<Vec<Cell>>;

#[allow(dead_code)]
fn print_map(map: &Map, direction: Direction) {
    for line in map {
        for cell in line {
            let c = match cell.kind {
                CellKind::Empty => {
                    if cell.visited { '.' }
                    else { '.' }
                },
                CellKind::Blocked => '#',
                CellKind::Us => {
                    match direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    }
                }
            };
            print!("{c}");
        }
        print!("\n");
    }
}

fn get_position(map: &Map) -> Option<(usize, usize)> {
    for (i, line) in map.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if cell.kind == CellKind::Us { return Some((i, j)) }
        }
    }
    return None;
}

fn restore_map(map: &mut Map, initial_position: (usize, usize)) {
    if let Some(position) = get_position(map) {
        let (j, i) = position;
        map[j][i] = Cell{ kind: CellKind::Empty, visited: true };
    }
    let (j, i) = initial_position;
    map[j][i] = Cell { kind: CellKind::Us, visited: true };
}

fn evolve(map: &mut Map, position: (usize, usize), direction: Direction, width: usize, height: usize) -> Option<((usize, usize), Direction)> {
    let (j, i) = position;
    match direction {
        Direction::Up => {
            // Count how many empty positions there are upwards (i.e. j towards zero)
            let mut dist = 0;
            for dj in 0..j {
                let next_cell = &mut map[j - dj - 1][i];
                match next_cell.kind {
                    CellKind::Empty => {
                        dist += 1;
                        next_cell.visited = true;
                    } 
                    _ => break,
                }
            }
            // Update map accordingly
            map[j][i] = Cell { kind: CellKind::Empty, visited: true };
            if j - dist == 0 { return None; }
            map[j - dist][i] = Cell { kind: CellKind::Us, visited: true };
            return Some(((j - dist, i), Direction::Right));
        }
        Direction::Down => {
            // Count how many empty positions there are upwards (i.e. j towards zero)
            let mut dist = 0;
            for dj in 1..(height-j) {
                let next_cell = &mut map[j + dj][i];
                match next_cell.kind {
                    CellKind::Empty => {
                        dist += 1;
                        next_cell.visited = true;
                    } 
                    _ => break,
                }
            }
            // Update map accordingly
            map[j][i] = Cell { kind: CellKind::Empty, visited: true };
            if j + dist == height-1 { return None; }
            map[j + dist][i] = Cell { kind: CellKind::Us, visited: true };
            return Some(((j + dist, i), Direction::Left));
        }
        Direction::Left => {
            // Count how many empty positions there are upwards (i.e. j towards zero)
            let mut dist = 0;
            for di in 0..i {
                let next_cell = &mut map[j][i - di - 1];
                match next_cell.kind {
                    CellKind::Empty => {
                        dist += 1;
                        next_cell.visited = true;
                    } 
                    _ => break,
                }
            }
            // Update map accordingly
            map[j][i] = Cell { kind: CellKind::Empty, visited: true };
            if i - dist == 0 { return None; }
            map[j][i - dist] = Cell {kind: CellKind::Us, visited: true };
            return Some(((j, i - dist), Direction::Up));
        }
        Direction::Right => {
            // Count how many empty positions there are upwards (i.e. j towards zero)
            let mut dist = 0;
            for di in 1..(width-i) {
                let next_cell = &mut map[j][i + di];
                match next_cell.kind {
                    CellKind::Empty => {
                        dist += 1;
                        next_cell.visited = true;
                    } 
                    _ => break,
                }
            }
            // Update map accordingly
            map[j][i] = Cell { kind: CellKind::Empty, visited: true };
            if i + dist == width-1 { return None; }
            map[j][i + dist] = Cell { kind: CellKind::Us, visited: true };
            return Some(((j, i + dist), Direction::Down));
        }
    }
}

fn part_1(map: &mut Map, initial_position: (usize, usize), initial_direction: Direction, width: usize, height: usize) {
    let mut position = initial_position;
    let mut direction = initial_direction;
    let mut visited = 0;
    loop {
        let new_status = evolve(map, position, direction, width, height);
        if let Some((new_position, new_direction)) = new_status {
            position = new_position;
            direction = new_direction;
        } else {
            // Count how many visited places
            for line in map {
                for cell in line {
                    if cell.visited { visited += 1; }
                }
            }
            break;
        }
    }
    println!("Number of visited positions = {visited}");
}

#[allow(dead_code)]
fn part_2(map: &mut Map, initial_position: (usize, usize), initial_direction: Direction, width: usize, height: usize) {
    let mut num_loops = 0;
    for j in 0..height {
        for i in 0..width {
            let cell = map[j][i];
            if cell.kind != CellKind::Empty { continue };
            map[j][i] = Cell{ kind: CellKind::Blocked, visited: false };
            let mut visited_positions: Vec<((usize, usize), Direction)> = Vec::new();
            visited_positions.push((initial_position, initial_direction));
            let mut position = initial_position;
            let mut direction = initial_direction;
            loop {
                let new_status = evolve(map, position, direction, width, height);
                if let Some((new_position, new_direction)) = new_status {
                    position = new_position;
                    direction = new_direction;
                    if !visited_positions.contains(&(new_position, new_direction)) {
                        visited_positions.push((new_position, new_direction))
                    } else {
                        num_loops += 1;
                        break;
                    }
                } else { break; }
            }
            map[j][i] = Cell { kind: CellKind::Empty, visited: false };
            restore_map(map, initial_position);
        }
    }
    println!("Number of loops = {num_loops}");
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("ERROR: could not read input");
    
    // Parse into 2D Vec
    let mut map: Map = Vec::new();
    let mut line: Vec<Cell> = Vec::new();
    for c in contents.chars() {
        match c {
            '.' => line.push(Cell { kind:CellKind::Empty, visited: false }),
            '#' => line.push(Cell { kind:CellKind::Blocked, visited: false }),
            '^' => line.push(Cell { kind:CellKind::Us, visited: true }),
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
    part_1(&mut map, initial_position, initial_direction, width, height);
    restore_map(&mut map, initial_position);
    part_2(&mut map, initial_position, initial_direction, width, height);
}