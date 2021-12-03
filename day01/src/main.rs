

fn main() -> anyhow::Result<()> {
    one()?;
    two()
}

fn one() -> anyhow::Result<()> {
    let s = include_str!("../input");

    let mut increased = 0;
    let mut last = 0;

    for line in s.lines() {
        let new = line.parse::<i32>().unwrap();
        if new > last && last != 0 {
            increased += 1;
        }

        last = new;
    }

    println!("increased: {}", increased);

    Ok(())
}



fn two() -> anyhow::Result<()> {
    let s = include_str!("../input");

    let mut window_sums: Vec<i32> = vec![];
    let mut window = vec![0; 3];
    let mut idx = 0;

    for line in s.lines() {
        let new = line.parse::<i32>().unwrap();
        window[idx] = new;

        window_sums.push(window.iter().fold(0, |acc, x| acc + x));

        idx = (idx + 1) % 3;
    }


    let mut increased = 0;
    let mut last = 0;

    for &new in window_sums.iter().skip(2) {
        if new > last && last != 0 {
            increased += 1;
        }

        last = new;
    }

    println!("increased: {}", increased);

    Ok(())
}