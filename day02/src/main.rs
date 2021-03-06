use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    one()?;
    two()
}

fn input() -> Vec<(String, i32)> {
    let s = include_str!("../input");
    s.lines().map(|line| {
        let line: (&str, &str) = line.split(" ").collect_tuple().unwrap();
        (String::from(line.0), line.1.parse::<i32>().unwrap())
    }).collect()
}

fn one() -> anyhow::Result<()> {
    let mut horizontal = 0;
    let mut vertical = 0;
    
    for (dir, n) in input() {
        match dir.as_str() {
            "forward" => { horizontal += n; }
            "up" => { vertical -= n; }
            "down" => { vertical += n; }
            _ => {}
        }
    }

    println!("{}", horizontal * vertical);

    Ok(())
}



fn two() -> anyhow::Result<()> {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    for (dir, n) in input() {
        match dir.as_str() {
            "forward" => {
                horizontal += n;
                vertical += aim * n;
            }
            "up" => { aim -= n; }
            "down" => { aim += n; }
            _ => {}
        }
    }

    println!("{}", horizontal * vertical);

    Ok(())
}
