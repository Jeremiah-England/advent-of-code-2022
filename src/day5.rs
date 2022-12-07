use std::collections::HashMap;

fn read_till_empty_line() -> Vec<String> {
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.trim().is_empty() {
            break;
        }
        lines.push(line);
    }
    lines
}

// Parse the ASCII stacks into a vector of stacks.
//
// Example Stacks:
//
//     [P]                 [C] [C]    
//     [W]         [B]     [G] [V] [V]
//     [V]         [T] [Z] [J] [T] [S]
//     [D] [L]     [Q] [F] [Z] [W] [R]
//     [C] [N] [R] [H] [L] [Q] [F] [G]
// [F] [M] [Z] [H] [G] [W] [L] [R] [H]
// [R] [H] [M] [C] [P] [C] [V] [N] [W]
// [W] [T] [P] [J] [C] [G] [W] [P] [J]
//  1   2   3   4   5   6   7   8   9 
fn parse_stacks(mut lines: Vec<String>) -> HashMap<u8, Vec<char>> {
    lines.reverse();
    let stack_numbers: Vec<u8> = lines.get(0).unwrap().trim().split("   ").map(|x| x.trim().parse().unwrap()).collect();
    let mut stacks: HashMap<u8, Vec<char>> = HashMap::new();
    for stack_number in &stack_numbers {
        stacks.insert(*stack_number, Vec::new());
    }
    for line in lines.iter().skip(1) {
        for position in 0..line.len() {
            if position % 4 == 1 {
                let stack_number = stack_numbers.get(position / 4).unwrap();
                let stack = stacks.get_mut(&stack_number).unwrap();
                let stack_char = line.chars().nth(position).unwrap();
                if stack_char != ' ' {
                    stack.push(stack_char);
                }
            }
        }
    }
    return stacks
}

fn move_stack_items(amount: &u8, from: &u8, to: &u8, stacks: &mut HashMap<u8, Vec<char>>) {
    let from_stack = stacks.get_mut(from).unwrap();
    let temp_stack = from_stack.split_off(from_stack.len() - *amount as usize);
    let to_stack = stacks.get_mut(to).unwrap();
    // temp_stack.reverse();  // For the first part of the challenge.
    to_stack.append(&mut temp_stack.clone());
}

// Example move_line is 'move 2 from 4 to 9'
fn parse_move(move_line: &String) -> (u8, u8, u8) {
    let mut words = move_line.trim().split(" ");
    let amount = words.nth(1).unwrap().parse().unwrap();
    let from = words.nth(1).unwrap().parse().unwrap();
    let to = words.nth(1).unwrap().parse().unwrap();
    return (amount, from, to);
}

fn print_stack_tops(stacks: &HashMap<u8, Vec<char>>) {
    let mut stack_tops: Vec<char> = Vec::new();
    for i in 1..(stacks.len() + 1) {
        let stack = stacks.get(&(i as u8)).unwrap();
        if stack.len() > 0 {
            stack_tops.push(stack[stack.len() - 1]);
        } else {
            stack_tops.push(' ');
        }
    }
    println!("{}", stack_tops.iter().collect::<String>());
}

pub fn solve() {
    let stacks_lines = read_till_empty_line();
    let mut stacks = parse_stacks(stacks_lines);

    let mut line = String::new();
    loop {
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.trim().is_empty() {
            break;
        }
        let (amount, from, to) = parse_move(&line);
        move_stack_items(&amount, &from, &to, &mut stacks);
    }
    print_stack_tops(&stacks);
}
