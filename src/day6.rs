use std::io;
use std::collections::HashSet;
// See https://stackoverflow.com/questions/40848918/are-there-queue-and-stack-collections-in-rust
use std::collections::VecDeque;


fn has_duplicates(vector: &VecDeque<char>) -> bool {
    let set: HashSet<&char> = HashSet::from_iter(vector.iter());
    return set.len() != vector.len()
}


fn find_uniq_index(string: &String) -> usize {
    let mut previous_four: VecDeque<char> = VecDeque::new();
    for (i, character) in string.chars().enumerate() {
        previous_four.push_back(character);
        if previous_four.len() == 4 {  // (Part 1)
        // if previous_four.len() == 14 {  // (Part 2)
            if !has_duplicates(&previous_four) {
                return i + 1;
            }
            previous_four.pop_front();
        }
    }
    panic!("Could not find index where enough preceding characters were unique.")
}


pub fn solve() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {break}
                println!("{}", find_uniq_index(&input));
            }
            Err(error) => println!("error: {error}"),
        }
    }
}
