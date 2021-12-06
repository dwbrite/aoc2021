#![feature(drain_filter)]
#![feature(slice_take)]

use std::collections::HashMap;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    one()?;
    two()
}
fn input() -> Vec<usize> {
    let s = include_str!("../input");
    s.lines().take(1).collect::<String>().split(',').map(|s| s.parse::<usize>().unwrap()).collect()
    // println!("{:?}", split);
    // split.iter().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn one() -> anyhow::Result<()> {
    let mut fish = input();

    for sim in 0..80 {
        let mut fish_to_add = vec![];
        for lifetime in &mut fish {
            if *lifetime == 0 {
                *lifetime = 7;
                fish_to_add.push(8usize);
            }
            *lifetime -=1;
        }
        fish.append(&mut fish_to_add);
    }

    println!("{:?}", fish.len());
    Ok(())
}


fn two() -> anyhow::Result<()> {
    let mut fish = input();

    let mut lifetimes = vec![0u128; 9];
    for idx in fish {
        lifetimes[idx as usize] += 1;
    }

    for sim in 0..256 {
        let mut new_lifetimes = vec![0u128; 9];

        for idx in 1..9 {
            new_lifetimes[idx-1] = lifetimes[idx];
        }
        new_lifetimes[6] += lifetimes[0];
        new_lifetimes[8] = lifetimes[0];

        lifetimes = new_lifetimes;
    }

    let mut ct = 0;
    for count in lifetimes {
        ct += count
    }

    println!("{:?}", ct);
    Ok(())
}
