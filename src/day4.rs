use regex::Regex;

pub fn a(data: &str) {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut count = 0;
    for line in data.lines() {
        let captures = re.captures(line).expect("No regex matches!");
        let first_low: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let first_high: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let second_low: usize = captures.get(3).unwrap().as_str().parse().unwrap();
        let second_high: usize = captures.get(4).unwrap().as_str().parse().unwrap();

        if first_low <= second_low && first_high >= second_high
            || second_low <= first_low && second_high >= first_high {
                count += 1;
            }
    }
    println!("Count: {count}");
}

pub fn b(data: &str) {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut count = 0;
    for line in data.lines() {
        let captures = re.captures(line).expect("No regex matches!");
        let first_low: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let first_high: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let second_low: usize = captures.get(3).unwrap().as_str().parse().unwrap();
        let second_high: usize = captures.get(4).unwrap().as_str().parse().unwrap();

        if second_low >= first_low  && second_low <= first_high
            || second_high >= first_low && second_high <= first_high
            || first_low >= second_low && first_low <= second_high
            || first_high >= second_low && first_high <= second_high
        {
            count += 1;
        }
    }
    println!("Count: {count}");

}