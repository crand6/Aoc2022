pub fn a(data: &str) {
    let groups = data.split("\n\n").collect::<Vec<&str>>();
    let mut max_sum = 0;
    for (_, group) in groups.iter().enumerate() {
        let sum: usize = group.split('\n').map(|x| x.parse::<usize>().unwrap_or(0)).sum();
        if sum > max_sum {
            max_sum = sum;
        }
    }
    println!("Max_sum: {}", max_sum);
}

pub fn b(data: &str) {
    let groups = data.split("\n\n").collect::<Vec<&str>>();
    let mut all_sums = vec![];
    for (_, group) in groups.iter().enumerate() {
        let sum: usize = group.split('\n').map(|x| x.parse::<usize>().unwrap_or(0)).sum();
        all_sums.push(sum);
    }
    all_sums.sort();
    all_sums.reverse();
    let top_three_sums: usize = all_sums[0..3].iter().sum();
    println!("top_three_sums: {}", top_three_sums);
}