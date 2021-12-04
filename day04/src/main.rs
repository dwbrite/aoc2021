#![feature(drain_filter)]
#![feature(slice_take)]

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    one()?;
    two()
}

struct BingoBoard {
    board: Vec<Vec<(String, bool)>>,
}

impl BingoBoard {
    fn check_called(&mut self) -> bool {
        self.check_vertical() || self.check_horizontal()
    }

    fn mark_next(&mut self, s: String) {
        for row in &mut self.board {
            for mut num in row {
                if num.0 == s {
                    num.1 = true;
                }
            }
        }
    }

    fn check_vertical(&mut self) -> bool {
        for x in 0..5 {
            let mut count = 0;
            for y in 0..5 {
                if !self.board[y][x].1 {
                    break;
                }
                count+=1;
            }
            if count == 5 {
                return true
            }
        }

        false
    }

    fn check_horizontal(&self) -> bool {
        for y in 0..5 {
            let mut count = 0;
            for x in 0..5 {
                if !self.board[y][x].1 {
                    break;
                }
                count+=1;
            }
            if count == 5 {
                return true
            }
        }

        false
    }
}

fn input() -> (Vec<String>, Vec<BingoBoard>) {
    let s = include_str!("../input");

    let order: Vec<String> = s.lines().take(1).collect::<String>().split(',').map(|s| s.to_string()).collect();

    let lines: Vec<String> = s.lines().skip(2).map(|s| s.to_string()).collect();
    let mut boards = vec![];

    for chunk in lines.chunks(6) {
        let mut board = vec![];
        for line in chunk {
            let row: Vec<(String, bool)> = line.split(' ').filter(|&s| s != "").map(|s| (s.to_string(), false)).collect();
            board.push(row)
        }
        boards.push(BingoBoard { board });
    }

    (order, boards)
}

fn one() -> anyhow::Result<()> {
    let (called, mut boards) = input();

    let (winner, last_call) = sim(&called, &mut boards);

    let mut sum = 0;
    for row in &boards[winner].board {
        for n in row {
            if !n.1 {
                sum += n.0.parse::<usize>().unwrap();
            }
        }
    }

    println!("{:?}", sum * last_call);
    Ok(())
}

fn sim(called: &Vec<String>, boards: &mut Vec<BingoBoard>) -> (usize, usize) {
    for i in 0..called.len() {
        for board in boards.iter_mut() {
            board.mark_next(called[i].clone())
        }

        for (idx, board) in boards.iter_mut().enumerate() {
            if board.check_called() {
                return (idx, called[i].parse::<usize>().unwrap())
            }
        }
    }

    (0, 0)
}

fn sim2(called: &Vec<String>, boards: &mut Vec<BingoBoard>) -> (usize, usize) {
    let mut last_winner = (0, 0);

    for i in 0..called.len() {
        for board in boards.iter_mut() {
            board.mark_next(called[i].clone())
        }

        let mut to_remove = vec![];

        for (idx, board) in boards.iter_mut().enumerate() {
            if board.check_called() {
                let n = called[i].parse::<usize>().unwrap();
                last_winner = (idx, called[i].parse::<usize>().unwrap());
                to_remove.push(idx); // sus af
                if n == 0 {
                    println!("bepis");
                }
            }
        }

        for idx in to_remove.iter().sorted().rev() {
            if boards.len() > 1 {
                boards.remove(*idx);
            } else {
                return last_winner
            }
        }
    }

    last_winner
}



fn two() -> anyhow::Result<()> {
    let (called, mut boards) = input();

    let (winner, last_call) = sim2(&called, &mut boards);

    let mut sum = 0;
    for row in &boards[winner].board {
        for n in row {
            if !n.1 {
                sum += n.0.parse::<usize>().unwrap();
            }
        }
    }

    println!("{:?}", sum * last_call);
    Ok(())
}
