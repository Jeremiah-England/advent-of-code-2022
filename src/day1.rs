use std::io;

fn read_integer_block() -> Vec<u32> {
    let mut input = String::new();
    let mut vec = Vec::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            break;
        }
        vec.push(input.trim().parse().unwrap_or_else(|_error| panic!("Not an integer '{}'.", input.as_str())));
    }
    vec
}

pub fn solve() {
    let mut calarie_sums: Vec<u32> = Vec::new();
    loop {
        let calaries = read_integer_block();
        if calaries.is_empty() {
            break;
        } else {
            let sum: u32 = calaries.iter().sum();
            calarie_sums.push(sum);
        }
    }
    calarie_sums.sort();
    println!("{}", calarie_sums.pop().unwrap() + calarie_sums.pop().unwrap() + calarie_sums.pop().unwrap())
}
