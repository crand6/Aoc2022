use regex::Regex;

pub fn a(data: &str) {
    // Initialize the ship manually instead of parsing from file
    let mut ship: [Vec<char>; 9] = [
        vec!['L', 'N', 'W', 'T', 'D'],
        vec!['C', 'P', 'H'],
        vec!['W', 'P', 'H', 'N', 'D', 'G', 'M', 'J'],
        vec!['C', 'W', 'S', 'N', 'T', 'Q', 'L'],
        vec!['P', 'H', 'C', 'N'],
        vec!['T', 'H', 'N', 'D', 'M', 'W', 'Q', 'B'],
        vec!['M', 'B', 'R', 'J', 'G', 'S', 'L'],
        vec!['Z', 'N', 'W', 'G', 'V', 'B', 'R', 'T'],
        vec!['W', 'G', 'D', 'N', 'P', 'L']
    ];

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in data.lines().skip(10) {
        let captures = re.captures(line).expect("No regex matches!");
        let count = captures.get(1).unwrap().as_str().parse().unwrap();
        let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to: usize = captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        for _ in 0u32..count {
            let c = ship.get_mut(from).unwrap().pop().unwrap();
            ship.get_mut(to).unwrap().push(c);
        }
    }
    for v in ship {
        let c = v.last().unwrap();
        print!("{c}");
    }
    println!();

}

pub fn b(data: &str) {
    // Initialize the ship manually instead of parsing from file
    let mut ship: [Vec<char>; 9] = [
        vec!['L', 'N', 'W', 'T', 'D'],
        vec!['C', 'P', 'H'],
        vec!['W', 'P', 'H', 'N', 'D', 'G', 'M', 'J'],
        vec!['C', 'W', 'S', 'N', 'T', 'Q', 'L'],
        vec!['P', 'H', 'C', 'N'],
        vec!['T', 'H', 'N', 'D', 'M', 'W', 'Q', 'B'],
        vec!['M', 'B', 'R', 'J', 'G', 'S', 'L'],
        vec!['Z', 'N', 'W', 'G', 'V', 'B', 'R', 'T'],
        vec!['W', 'G', 'D', 'N', 'P', 'L']
    ];

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in data.lines().skip(10) {
        let captures = re.captures(line).expect("No regex matches!");
        let count: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to: usize = captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        let [src, dest] = ship.get_many_mut([from, to]).unwrap();
        let x = (*src).drain((src.len() - count)..);
        dest.extend(x);
    }
    for v in ship {
        let c = v.last().unwrap();
        print!("{c}");
    }
    println!();

}