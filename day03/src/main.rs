#![feature(drain_filter)]

fn main() -> anyhow::Result<()> {
    one()?;
    two()
}

fn input() -> Vec<String> {
    let s = include_str!("../input");
    s.lines().map(|line| {
        String::from(line)
    }).collect()
}

fn find_most_common(idx: usize, input: &Vec<String>) -> char {
    let count0 = input.iter().filter(|&s| s.chars().nth(idx).unwrap() == '0').count();
    let count1 = input.len() - count0;

    if count0 > count1 {
        return '0'
    }
    '1'
}

fn one() -> anyhow::Result<()> {
    let input = input();

    let mut gamma = String::new();

    for i in 0..12 {
        gamma.push(find_most_common(i, &input));
    }

    let epsilon: String = gamma.chars().map(|c| {
        // clearly way more readable than an if-else. It saves a whole 4 loc!
        ((c as u8 - 0x30) ^ 1 + 0x30) as char
    }).collect();

    let g = usize::from_str_radix(&gamma, 2)?;
    let e = usize::from_str_radix(&epsilon, 2)?;

    println!("{}", g*e);
    Ok(())
}



fn two() -> anyhow::Result<()> {
    let input = input();

    let mut o2gen = input.clone();
    for i in 0..12 {
        let most_common = find_most_common(i, &o2gen);
        o2gen.drain_filter(move |s| { s.chars().nth(i).unwrap() != most_common });

        if o2gen.len() == 1 { break; }
    }

    let mut co2scrub = input.clone();
    for i in 0..12 {
        let most_common = find_most_common(i, &co2scrub);
        co2scrub.drain_filter(move |s| { s.chars().nth(i).unwrap() == most_common });

        if co2scrub.len() == 1 { break; }
    }

    let o = usize::from_str_radix(&o2gen[0], 2)?;
    let c = usize::from_str_radix(&co2scrub[0], 2)?;

    println!("{}", o*c);

    Ok(())
}
