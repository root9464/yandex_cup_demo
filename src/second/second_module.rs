use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[path = "../utils/input.rs"]
mod input;
/// Функция для вычисления времени, когда Алексей проснётся
pub fn alarm_clock(x: i32, k: usize, times: Vec<i32>) -> i32 {
    let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut count: i32 = 0;
    let mut last_wakeup: i32 = -1;

    // Добавляем начальные моменты звонков всех N будильников в кучу (мин-куча через Reverse)
    for &t in &times {
        pq.push(std::cmp::Reverse(t));
    }

    while count < k.try_into().unwrap() {
        let std::cmp::Reverse(current_time) = pq.pop().unwrap();
        // Если звонок не совпадает с предыдущим моментом, считаем его

        if current_time != last_wakeup {
            count += 1;
            last_wakeup = current_time
        }

        // Добавляем следующий звонок для текущего будильника (через X минут)
        pq.push(std::cmp::Reverse(current_time + x));
    }
    return last_wakeup;
}

/// Функция для чтения входных данных
pub fn read_input_data() -> (i32, usize, Vec<i32>) {
    // Чтение первой строки — количества будильников, интервала и количества звонков для пробуждения
    let first_line = input::read_input("Введите количество будильников, интервал и количество звонков для пробуждения (через пробел):");
    let first_line: Vec<i32> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = first_line[0] as usize; // Количество будильников (N)
    let x = first_line[1]; // Интервал между звонками (X)
    let k = first_line[2] as usize; // Количество звонков для пробуждения (K)

    // Чтение второй строки — моментов времени будильников
    let times_line = input::read_input("Введите моменты времени для каждого будильника:");
    let times: Vec<i32> = times_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Проверка: Если количество моментов времени не соответствует N, программа не работает
    assert_eq!(
        n,
        times.len(),
        "Количество моментов времени должно быть равно N!"
    );

    (x, k, times)
}
