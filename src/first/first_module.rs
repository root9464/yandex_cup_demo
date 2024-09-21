use std::io;

use crate::input;

pub fn jewelry_search() -> io::Result<i32> {
    let stones: String = input::read_input("Камни: ");
    let jewelrys: String = input::read_input("Драгоценности: ");
    let mut counter: i32 = 0;
    for stone in stones.chars() {
        if jewelrys.contains(stone) {
            counter += 1;
        }
    }
    Ok(counter)
}
