use std::fs;
use std::collections::HashMap;

#[allow(dead_code)]
struct Antenna {
    frequency: char,
}

type Map = Vec<Vec<Option<Antenna>>>;
type Location = (i32, i32);

fn get_antinodes(a: Location, b: Location) -> [Location; 2] {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    return [(a.0 + dx, a.1 + dy), (b.0 - dx, b.1 - dy)];
}

fn is_within(loc: &Location, map: &Map) -> bool {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    return loc.0 >= 0 && loc.0 < width && loc.1 >= 0 && loc.1 < height;
}

fn part_1(map: &Map, frequency_locations: &HashMap<char, Vec<Location>>) {
    let mut antinodes: Vec<Location> = Vec::new();
    for (_, locs) in frequency_locations {
        for (i, loc) in locs.iter().enumerate() {
            for locb in &locs[i+1..] {
                let [a1, a2] = get_antinodes(*loc, *locb);
                if !antinodes.contains(&a1) && is_within(&a1, &map) { antinodes.push(a1); }
                if !antinodes.contains(&a2) && is_within(&a2, &map) { antinodes.push(a2); }
            }
        }
        
    }
    println!("Number of antinodes = {}", antinodes.len());
}

fn get_antinodes_2(a: Location, b: Location, map: &Map) -> Vec<Location> {
    let mut antinodes = Vec::new();
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let mut i = 0;
    // a + i*delta
    loop {
        let antinode: Location = (a.0 + i*dx, a.1 + i*dy);
        if is_within(&antinode, &map) { antinodes.push(antinode); }
        else { break; }
        i += 1;
    }
    // b - i*delta
    i = 0;
    loop {
        let antinode: Location = (b.0 - i*dx, b.1 - i*dy);
        if is_within(&antinode, &map) { antinodes.push(antinode); }
        else { break; }
        i += 1;
    }
    return antinodes;
}

fn part_2(map: &Map, frequency_locations: &HashMap<char, Vec<Location>>) {
    let mut antinodes: Vec<Location> = Vec::new();
    for (_, locs) in frequency_locations {
        for (i, loc) in locs.iter().enumerate() {
            for locb in &locs[i+1..] {
                let antis = get_antinodes_2(*loc, *locb, &map);
                for a in antis {
                    if !antinodes.contains(&a) { antinodes.push(a); }
                }
            }
        }
        
    }
    println!("Number of antinodes with resonance = {}", antinodes.len());
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("");
    let mut map: Map = Vec::new();
    let mut frequency_locations: HashMap<char, Vec<Location>> = HashMap::new();
    let mut line: Vec<Option<Antenna>> = Vec::new();
    for c in contents.chars() {
        match c {
            '\n' => {
                map.push(line);
                line = Vec::new();
            },
            '.' => line.push(None),
            _ => {
                let j = map.len() as i32;
                let i = line.len() as i32;
                if let Some(locs) = frequency_locations.get_mut(&c) {
                    locs.push((i, j));
                } else { 
                    let mut locs = Vec::new();
                    locs.push((i, j));
                    frequency_locations.insert(c, locs);
                };
                line.push(Some(Antenna{frequency: c}))
            },
        }
    }
    part_1(&map, &frequency_locations);
    part_2(&map, &frequency_locations);
}