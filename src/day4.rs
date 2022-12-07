use std::cmp;
use std::io;

// fn range_contains_other(range: (&u8, &u8), other: (&u8, &u8)) -> bool {
//     return (range.0 <= other.0) && (other.1 <= range.1)
// }
//
// fn ranges_completely_overlap(range1: (&u8, &u8), range2: (&u8, &u8)) -> bool {
//     // One of the ranges entirely contains the other.
//     return range_contains_other(range1, range2) || range_contains_other(range2, range1)
// }

fn ranges_overlap(range1: (&u8, &u8), range2: (&u8, &u8)) -> bool {
    return cmp::max(range1.0, range2.0) <= cmp::min(range1.1, range2.1);
}

fn vec_to_pair<T>(vector: &Vec<T>) -> (&T, &T) {
    assert!(vector.len() == 2);
    return (vector.get(0).unwrap(), vector.get(1).unwrap());
}

pub fn solve() {
    let mut line = String::new();
    let mut count = 0;
    loop {
        line.clear();
        match io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let elf_pair: Vec<Vec<u8>> = line
                    .trim()
                    .split(',')
                    .map(|s| s.split('-').map(|num| num.parse().unwrap()).collect())
                    .collect();
                assert!(
                    elf_pair.len() == 2,
                    "There should just be two elfs on a given line."
                );
                let elf1 = vec_to_pair(elf_pair.get(0).unwrap());
                let elf2 = vec_to_pair(elf_pair.get(1).unwrap());
                // if ranges_completely_overlap(elf1, elf2) {
                //     count += 1;
                // }
                let overlaps = ranges_overlap(elf1, elf2);
                if overlaps {
                    count += 1;
                }
                let trim_line = line.trim();
                println!("{trim_line}\t{overlaps}")
            }
            Err(error) => {
                println!("error: {error}");
                break;
            }
        }
    }
    println!("{count}");
}
