use crate::input;

pub fn cart_game(k: i32, cards: Vec<i32>) -> &'static str {
    let mut vasya_score = 0;
    let mut petya_score = 0;

    for card in cards {
        match card {
            card if card % 3 == 0 && card % 5 == 0 => continue, // Никто не получает очков
            card if card % 5 == 0 => vasya_score += 1,          // Вася получает очко
            card if card % 3 == 0 => petya_score += 1,          // Петя получает очко
            _ => continue,                                      // Никто не получает очков
        }

        if vasya_score >= k {
            return "Vasya";
        } else if petya_score >= k {
            return "Petya";
        }
    }

    match vasya_score.cmp(&petya_score) {
        std::cmp::Ordering::Greater => "Vasya",
        std::cmp::Ordering::Less => "Petya",
        std::cmp::Ordering::Equal => "Draw",
    }
}

pub fn read_input_data() -> (i32, Vec<i32>) {
    let first_line: String =
        input::read_input("Введите количество очков K и количество карточек N (через пробел):");
    let first_line: Vec<i32> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let k: i32 = first_line[0]; // Количество очков (K)
    let n: usize = first_line[1] as usize; // Количество карточек (N)

    // Чтение карточек Пети
    let cards_petya_line: String =
        input::read_input("Введите карточки, которые Петя выкладывает на стол:");
    let cards_petya: Vec<i32> = cards_petya_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    assert_eq!(
        n,
        cards_petya.len(),
        "Количество карточек должно быть равно N!"
    );

    (k, cards_petya)
}
