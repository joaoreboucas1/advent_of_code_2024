use std::fs;
use std::collections::HashMap;

type Id = usize;
type Memory = Vec<Option<Id>>;

#[allow(dead_code)]
fn print_memory(m: &Memory) {
    for block in m {
        if let Some(id) = block {
            print!("[{id}]");
        } else  {
            print!(".");
        }
    }
    println!();
}

fn build_memory(contents: &String) -> Memory {
    let mut m: Memory = Vec::new();
    for (i, c) in contents.chars().enumerate() {
        let size = c.to_digit(10).unwrap();
        if i%2 == 0 {
            // Size of chunk
            let id = i/2;
            for _ in 0..size {
                m.push(Some(id));
            }
        } else {
            for _ in 0..size {
                m.push(None);
            }
        }
    }
    return m;
}

fn ids_with_sizes(contents: &String) -> HashMap<usize, u32> {
    let mut map = HashMap::new();
    for (i, c) in contents.chars().enumerate() {
        let size = c.to_digit(10).unwrap();
        if i%2 == 0 {
            // Size of chunk
            let id = i/2;
            map.insert(id, size);
        }
    }
    return map;
}

fn first_free(m: &Memory, last_free: usize) -> usize {
    for (i, block) in m[last_free..].iter().enumerate() {
        if let None = block { return last_free + i; }
    }
    panic!("We should have free memory somewhere so this SHOULD be unreachable.");
}

fn is_defrag(m: &Memory, free: usize) -> bool {
    for block in m[free..].iter() {
        if let Some(_) = block { return false; }
    }
    return true;
}

fn defragment(m: &mut Memory) {
    let mut free = first_free(m, 0);
    let mem_size = m.len();
    for j in 0..mem_size {
        let i = mem_size - j - 1;
        if let Some(id) = m[i] {
            m[free] = Some(id);
            m[i] = None;
            free = first_free(m, free);
            if is_defrag(&m, free) { break; }
        } else {
            continue;
        }
    }
}

fn checksum(m: &Memory) -> usize {
    return m.iter()
            .enumerate()
            .filter(|(_, x)| matches!(x, Some(_)))
            .map(|(i, x)| i*x.unwrap())
            .sum();
}

fn part_1(contents: &String) {
    let mut memory = build_memory(contents);
    defragment(&mut memory);
    println!("Checksum = {}", checksum(&memory));
}

fn last_id(memory: &Memory) -> Id {
    for i in memory.iter().rev() {
        if let Some(id) = i { return *id; }
    }
    panic!("This should be unreachable");
}

fn find_id(memory: &Memory, id: Id) -> usize {
    // Returns index of chunk with id `id`
    for (index, i) in memory.iter().enumerate() {
        if let Some(_) = i {
            if i.unwrap() == id { return index; }
        }
    }
    println!("Cannot find id = {id}");
    panic!("Should be unreachable");
}

fn find_fitting_space(memory: &Memory, size: u32) -> Option<usize> {
    // Returns index and size of fitting space
    let mut i = 0;
    loop {
        if i == memory.len() { break; }
        if memory[i].is_none() {
            let index = i;
            while memory[i].is_none() { 
                i += 1;
                if i == memory.len() { break; }
            }
            let empty_size = i - index;
            if empty_size >= size as usize { return Some(index); }
            else { continue; }
        }
        i += 1;
    }
    return None;
}

fn defrag2(memory: &mut Memory, sizes: HashMap<usize, u32>) {
    let last_id = last_id(memory);
    for id in (0..=last_id).rev() {
        let index = find_id(memory, id);
        let size = sizes[&id];
        // Find where it can be placed in memory 
        let empty = find_fitting_space(memory, size);
        if empty.is_none() { println!("Cannot find space that accomodates id = {id}"); continue; }
        let empty = empty.unwrap();
        if empty > index { continue; }
        // Put the chunk in that empty space
        for i in empty..empty+size as usize {
            assert!(memory[i].is_none());
            memory[i] = Some(id);
        }
        // Empty the original space
        for i in index..index+size as usize {
            assert!(memory[i] == Some(id));
            memory[i] = None;
        }
    }
}

fn part_2(contents: &String) {
    let mut memory = build_memory(contents);
    let sizes = ids_with_sizes(contents);
    defrag2(&mut memory, sizes);
    println!("Checksum = {}", checksum(&memory));
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("");
    part_1(&contents);
    part_2(&contents);
}