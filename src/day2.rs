use std::io;


enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn char_to_shape(character: &str) -> Shape {
    match character {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Invalid shape character {character}.")
    }
}

//  "X means you need to lose, Y means you need to end the round in a draw, and Z means you need to
//  win. Good luck!"
fn char_to_win_status(character: &str) -> WinStatus {
    match character {
        "X" => WinStatus::Lose,
        "Y" => WinStatus::Draw,
        "Z" => WinStatus::Win,
        _ => panic!("Invalid win status character {character}.")
    }
}

// (1 for Rock, 2 for Paper, and 3 for Scissors)
fn shape_score(shape: Shape) -> u8 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

enum WinStatus {
    Win,
    Lose,
    Draw
}

fn shapes_to_win_statuses(shapes: &(Shape, Shape))  -> (WinStatus, WinStatus) {
    match shapes {
        (Shape::Rock, Shape::Rock) => (WinStatus::Draw, WinStatus::Draw),
        (Shape::Rock, Shape::Paper) => (WinStatus::Lose, WinStatus::Win),
        (Shape::Rock, Shape::Scissors) => (WinStatus::Win, WinStatus::Lose),
        (Shape::Paper, Shape::Rock) => (WinStatus::Win, WinStatus::Lose),
        (Shape::Paper, Shape::Paper) => (WinStatus::Draw, WinStatus::Draw),
        (Shape::Paper, Shape::Scissors) => (WinStatus::Lose, WinStatus::Win),
        (Shape::Scissors, Shape::Rock) => (WinStatus::Lose, WinStatus::Win),
        (Shape::Scissors, Shape::Paper) => (WinStatus::Win, WinStatus::Lose),
        (Shape::Scissors, Shape::Scissors) => (WinStatus::Draw, WinStatus::Draw),
    }
}

fn shape_win_to_shape(shape: &Shape, win: WinStatus)  -> Shape {
    // Given you know the other shape and you want the `win` WinStatus,
    // what shape should you choose?
    match (shape, win) {
        (Shape::Rock, WinStatus::Lose) => Shape::Scissors,
        (Shape::Rock, WinStatus::Draw) => Shape::Rock,
        (Shape::Rock, WinStatus::Win) => Shape::Paper,
        (Shape::Paper, WinStatus::Lose) => Shape::Rock,
        (Shape::Paper, WinStatus::Draw) => Shape::Paper,
        (Shape::Paper, WinStatus::Win) => Shape::Scissors,
        (Shape::Scissors, WinStatus::Lose) => Shape::Paper,
        (Shape::Scissors, WinStatus::Draw) => Shape::Scissors,
        (Shape::Scissors, WinStatus::Win) => Shape::Rock,
    }
}

// (0 if you lost, 3 if the round was a draw, and 6 if you won)
fn win_status_to_score(win_status: WinStatus) -> u8 {
    match win_status {
        WinStatus::Lose => 0,
        WinStatus::Draw => 3,
        WinStatus::Win => 6,
    } 
}


fn score_round(shapes: (Shape, Shape)) -> (u8, u8) {
    let win_statuses: (WinStatus, WinStatus) = shapes_to_win_statuses(&shapes);
    return (shape_score(shapes.0) + win_status_to_score(win_statuses.0), shape_score(shapes.1) + win_status_to_score(win_statuses.1));
}

pub fn solve() {
    let mut line = String::new();
    let mut your_total: u32 = 0;
    loop {
        line.clear();
        match io::stdin().read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break
                }
                let mut plays: Vec<&str> = line.trim().split(' ').collect();
                let desired_win_status = plays.pop().expect("Empty!");
                let their_play = plays.pop().expect("Only had one!");
                assert!(plays.is_empty());
                let their_shape = char_to_shape(their_play);
                let your_shape = shape_win_to_shape(&their_shape, char_to_win_status(desired_win_status));
                let scores = score_round((their_shape, your_shape));
                // let their_score = scores.0;
                // let your_score = scores.1;
                your_total += scores.1 as u32;
                // println!("{their_play} {your_play} -> {their_score}, {your_score}");
            }
            Err(error) => {
                println!("error: {error}");
                break
            },
        }
    }
    println!("{your_total}")
}
