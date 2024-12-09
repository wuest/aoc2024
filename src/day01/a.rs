use std::fs;

fn empty() -> (Vec<i32>, Vec<i32>)
{
    (vec![], vec![])
}

fn parse((a, b): (Vec<i32>, Vec<i32>), str_pair: &str) -> (Vec<i32>, Vec<i32>)
{
    let numbers = str_pair.split_whitespace()
                          .map(|num| num.parse().unwrap())
                          .collect::<Vec<i32>>();
    // Crash if the parse fails!
    ([a, vec![numbers[0].clone()]].concat(), [b, vec![numbers[1].clone()]].concat())
}

fn main()
{
    let path = "./src/day01/input.txt";

    let contents = fs::read_to_string(path)
        .expect("Cannot open {path}");
    let (left, right) = contents.lines()
                                .fold(empty(), |vecs, str_pair| parse(vecs, str_pair));

    let mut left = left.clone();
    let mut right = right.clone();
    left.sort();
    right.sort();

    let mut i = 0;
    let mut sum = 0;

    while i < left.len()
    {
        sum += (left[i] - right[i]).abs();
        i += 1;
    }

    println!("{:?}", sum);
}
