use std::collections::HashSet;

pub fn a(data: &str) {
    let char_vec: Vec<char> = data.chars().collect();
    let mut set: HashSet<char> = HashSet::with_capacity(4);
    let mut start_index = 0;
    loop {
        let slice = char_vec.get(start_index..start_index+4).unwrap();
        set.extend(slice);
        if set.len() == 4 {
            // +3 because we want the end of the "start of packet"
            // + 1 to adjust for 0-based vs 1-based indexing
            let solution = start_index + 4;
            println!("Found at {solution}!");
            break;
        }
        set.clear();
        start_index += 1;
    }
}

pub fn b(data: &str) {
    let char_vec: Vec<char> = data.chars().collect();
    let mut set: HashSet<char> = HashSet::with_capacity(14);
    let mut start_index = 0;
    loop {
        let slice = char_vec.get(start_index..start_index+14).unwrap();
        set.extend(slice);
        if set.len() == 14 {
            // +3 because we want the end of the "start of packet"
            // + 1 to adjust for 0-based vs 1-based indexing
            let solution = start_index + 14;
            println!("Found at {solution}!");
            break;
        }
        set.clear();
        start_index += 1;
    }
}