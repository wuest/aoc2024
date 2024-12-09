use std::fs;
use std::collections::HashMap;

fn parse(str_pair: &str) -> (i32, i32)
{
    let numbers = str_pair.split_whitespace()
                          .map(|num| num.parse().unwrap())
                          .collect::<Vec<i32>>();
    // Crash if the parse fails!
    (numbers[0], numbers[1])
}

fn main()
{
    let path = "./src/day01/input.txt";

    let contents = fs::read_to_string(path)
        .expect("Cannot open {path}");

    let lines = contents.lines();
    let mut left: Vec<i32> = vec![];
    let mut right: HashMap<i32, i32> = HashMap::new();
    for line in lines
    {
        let (l, r) = parse(line);
        left.push(l);
        right.entry(r).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut sum = 0;
    for i in left
    {
        sum += i * right.get(&i).unwrap_or(&0);
    }

    println!("{:?}", sum);
}
