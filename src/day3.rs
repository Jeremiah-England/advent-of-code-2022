use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
//
// Find the item type that appears in both compartments of each rucksack. What is the sum of the
// priorities of those item types?

fn create_chars_to_priority() -> HashMap<char, u8> {
    let mut map: HashMap<char, u8> = HashMap::new();
    for (i, character) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
    {
        map.insert(character, (i + 1) as u8);
    }
    return map;
}

// struct Rucksack {
//     front: Vec<char>,
//     back: Vec<char>,
// }
//
// impl Rucksack {
//     fn from_string(string: &String) -> Rucksack {
//         assert!(!(string.is_empty()));
//         let (front, back) = string.split_at(string.len() / 2);
//         Rucksack { front: front.chars().collect(), back: (back.chars().collect()) }
//     }
//
//     fn shared_character(&self) -> &char {
//         for front_char in &self.front {
//             for back_char in &self.back {
//                 if front_char == back_char {
//                     return front_char;
//                 }
//             }
//         }
//         panic!("No shared character found!");
//     }
// }

fn find_shared_character(strings: &Vec<String>) -> char {
    let mut sets: Vec<HashSet<char>> = strings
        .iter()
        .map(|string| HashSet::from_iter(string.chars()))
        .collect();
    let set1 = sets.pop().expect("Looks like the string vector was empty.");
    let mut shared_characters: Vec<&char> = set1
        .iter()
        .filter(|c| sets.iter().all(|s| s.contains(c)))
        .collect();
    let character = shared_characters
        .pop()
        .expect("There were no shared characters...");
    return character.to_owned();
}

pub fn solve() {
    let mut line = String::new();
    let priority_by_chars = create_chars_to_priority();
    let mut priority_sum: u16 = 0;
    let mut elves: Vec<String> = Vec::new();
    loop {
        line.clear();
        match io::stdin().read_line(&mut line) {
            Ok(num_bytes) => {
                if num_bytes == 0 {
                    break;
                }
                elves.push(line.trim().to_owned());
                if elves.len() == 3 {
                    let priority = priority_by_chars
                        .get(&find_shared_character(&elves))
                        .expect("Invalid character.");
                    priority_sum += *priority as u16;
                    elves.clear();
                }
                // let rucksack = Rucksack::from_string(&line);
                // let shared_character = rucksack.shared_character();
                // let priority = priority_by_chars.get(shared_character).expect("Invalid character.");
            }
            Err(error) => println!("error: {error}."),
        }
    }
    println!("{priority_sum}")
}
