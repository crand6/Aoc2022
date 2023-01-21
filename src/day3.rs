use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>
}

impl Rucksack {
    fn new(data: &str) -> Self {
        println!("{data}");
        assert!(data.len() % 2 == 0);
        let mid_index = data.len() / 2;
        let mut compartment1 = HashSet::with_capacity(mid_index);
        let mut compartment2 = HashSet::with_capacity(mid_index);

        for char in &data.chars().collect::<Vec<char>>()[0..mid_index] {
            compartment1.insert(*char);
        }
        for char in &data.chars().collect::<Vec<char>>()[mid_index..] {
            compartment2.insert(*char);
        }

        Self { compartment1, compartment2 }
    }


    fn get_common_priority(&self) -> usize {
        let intersection = self.compartment1
            .intersection(&self.compartment2)
            .copied()
            .collect::<Vec<char>>();
        assert_eq!(intersection.len(), 1);
       
        let intersection = *intersection.first().unwrap();
        println!("{intersection}");
        let mut priority: u32 = intersection.try_into().unwrap();
        if intersection.is_ascii_lowercase() {
            priority = priority - 0x61 + 1;
        }
        else {
            priority = priority - 0x41 + 27;
        }
       
        priority.try_into().unwrap()
    }
}

struct RucksackB {
    first: HashSet<char>,
    second: HashSet<char>,
    third: HashSet<char>
}

impl RucksackB {

    fn new(chunk: [&str; 3]) -> Self {
        let first = chunk[0].chars().collect::<HashSet<char>>();
        let second = chunk[1].chars().collect::<HashSet<char>>();
        let third = chunk[2].chars().collect::<HashSet<char>>();
        Self { first, second, third }
    }

    fn get_common_priority(&self) -> usize {
        let intersection = self.first
            .intersection(&self.second)
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&self.third)
            .copied()
            .collect::<Vec<char>>();
        assert_eq!(intersection.len(), 1);
       
        let intersection = *intersection.first().unwrap();
        println!("{intersection}");
        let mut priority: u32 = intersection.try_into().unwrap();
        if intersection.is_ascii_lowercase() {
            priority = priority - 0x61 + 1;
        }
        else {
            priority = priority - 0x41 + 27;
        }
       
        priority.try_into().unwrap()
    
    }

}
pub fn a(data: &str) {
    let priorities_sum: usize = data.lines()
        .map(
            |line|
                Rucksack::new(line)
                .get_common_priority())
        .sum();
    println!("Sum of priorities: {priorities_sum}");
    // for line in data.lines() {
        // let rs = Rucksack::new(line);
        // println!("{}", rs.get_common_priority());
        // break;
    // }
}

pub fn b(data: &str) {
    let priorities_sum: usize = data.lines()
        .array_chunks::<3>()
        .map(|chunk| RucksackB::new(chunk).get_common_priority())
        .sum();
    println!("Total sum: {priorities_sum}");
}