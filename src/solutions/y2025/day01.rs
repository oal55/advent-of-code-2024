use crate::common::io::read_file;

pub fn run(file_path: &str) -> (u64, u64) {    
    let nums: Vec<_> = read_file(file_path).lines()
        .map(parse)
        .collect();

    (
        turn_the_dial_and_shit(&nums),
        turn_the_dial_and_shit_2(&nums)
    )
}

fn parse(line: &str) -> i64 {
    let (sign_str, num) = line.split_at(1);
    let sign = match sign_str {
        "L" => -1,
        "R" => 1,
        _ => unreachable!("Bad sign {sign_str}")
    };
    return sign * num.parse::<i64>().unwrap();
}

fn turn_the_dial_and_shit(turns: &Vec<i64>) -> u64 {
    let mut num_zeros = 0;
    let mut dial_position = 50;
    for turn in turns {
        dial_position = (dial_position + turn) % 100;
        if dial_position == 0 {
            num_zeros += 1;
        }
    }
    num_zeros
}

fn turn_the_dial_and_shit_2(turns: &Vec<i64>) -> u64 {
    let mut num_zeros = 0;
    let mut dial_position: i64 = 50;
    for turn in turns.iter() {
        let mut turn = *turn;

        num_zeros += (turn.abs() / 100) as u64;
        turn %= 100;

        let fi_sign = dial_position.signum();
        let turned_raw = dial_position + turn;

        // We crossed zero. Eg -1 -> 3
        if fi_sign != 0 && fi_sign != turned_raw.signum() {
            num_zeros += 1;
        }
        // We may have crossed zeros on either side
        // -99 -> -103 or 95 -> 105
        num_zeros += (turned_raw.abs() / 100) as u64;
        
        dial_position = turned_raw % 100;
    }
    num_zeros
}
