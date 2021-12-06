#![feature(drain_filter)]
#![feature(slice_take)]

use std::collections::HashMap;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    one()?;
    two()
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
struct Coord {
    x: u32,
    y: u32,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Copy, Clone)]
struct Line(Coord, Coord);

impl Line {
}

fn input() -> Vec<Line> {
    let s = include_str!("../input");
    let s_lines: Vec<String> = s.lines().map(|s| s.to_string()).collect();

    let mut lines = vec![];

    for line in s_lines {
        let coords: Vec<String> = line.split(" -> ").map(|s| s.to_string()).collect();

        let mut c_line: Vec<Coord> = coords.iter().map(|s| {
            let coord: Vec<u32> = s.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
            Coord { x: coord[0], y: coord[1]}
        }).collect();

        lines.push(Line(c_line[0].clone(), c_line[1].clone()));
    }

    lines
}

fn one() -> anyhow::Result<()> {
    let vent_lines = input();

    let mut intersections: HashMap<Coord, u32> = HashMap::new();

    for vent in vent_lines {
        if vent.0.x == vent.1.x {
            let mut lower_bound = vent.0.y;
            let mut upper_bound = vent.0.y;
            if vent.0.y > vent.1.y {
                lower_bound = vent.1.y;
            } else {
                upper_bound = vent.1.y;
            }

            for y in lower_bound..=upper_bound {
                let coord = Coord {
                    x: vent.0.x,
                    y
                };

                if let Some(value) = intersections.get_mut(&coord) {
                    *value += 1;
                } else {
                    intersections.insert(coord, 1);
                }
            }
        } else if vent.0.y == vent.1.y {
            let mut lower_bound = vent.0.x;
            let mut upper_bound = vent.0.x;
            if vent.0.x > vent.1.x {
                lower_bound = vent.1.x;
            } else {
                upper_bound = vent.1.x;
            }

            for x in lower_bound..=upper_bound {
                let coord = Coord {
                    y: vent.0.y,
                    x
                };

                if let Some(value) = intersections.get_mut(&coord) {
                    *value += 1;
                } else {
                    intersections.insert(coord, 1);
                }
            }
        }
    }

    let ct = intersections.iter().filter(|(_, &c)| c > 1).count();

    println!("{:?}", ct);
    Ok(())
}


fn two() -> anyhow::Result<()> {
    let vent_lines = input();

    let mut intersections: HashMap<Coord, u32> = HashMap::new();

    for vent in vent_lines {
        if vent.0.x == vent.1.x {
            let mut lower_bound = vent.0.y;
            let mut upper_bound = vent.0.y;
            if vent.0.y > vent.1.y {
                lower_bound = vent.1.y;
            } else {
                upper_bound = vent.1.y;
            }

            for y in lower_bound..=upper_bound {
                let coord = Coord {
                    x: vent.0.x,
                    y
                };

                if let Some(value) = intersections.get_mut(&coord) {
                    *value += 1;
                } else {
                    intersections.insert(coord, 1);
                }
            }
        } else if vent.0.y == vent.1.y {
            let mut lower_bound = vent.0.x;
            let mut upper_bound = vent.0.x;
            if vent.0.x > vent.1.x {
                lower_bound = vent.1.x;
            } else {
                upper_bound = vent.1.x;
            }

            for x in lower_bound..=upper_bound {
                let coord = Coord {
                    y: vent.0.y,
                    x
                };

                if let Some(value) = intersections.get_mut(&coord) {
                    *value += 1;
                } else {
                    intersections.insert(coord, 1);
                }
            }
        } else {
            let mut left;
            let mut right;
            if vent.0.x < vent.1.x {
                left = vent.0;
                right = vent.1;
            } else {
                left = vent.1;
                right = vent.0;
            }

            println!("AAAAAA");

            if left.y > right.y {
                for i in 0..=(left.y - right.y) {
                    let coord = Coord {
                        x: left.x + i,
                        y: left.y - i,
                    };

                    println!("{:?}", coord);

                    if let Some(value) = intersections.get_mut(&coord) {
                        *value += 1;
                    } else {
                        intersections.insert(coord, 1);
                    }
                }
            } else {
                for i in 0..=(right.y - left.y) {
                    let coord = Coord {
                        x: left.x + i,
                        y: left.y + i,
                    };

                    if let Some(value) = intersections.get_mut(&coord) {
                        *value += 1;
                    } else {
                        intersections.insert(coord, 1);
                    }
                }
            }
        }
    }

    let ct = intersections.iter().filter(|(_, &c)| c > 1).count();

    println!("{:?}", ct);
    Ok(())
}
