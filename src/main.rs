#[path = "first/first_module.rs"]
mod first_module;

#[path = "./utils/input.rs"]
mod input;

#[path = "./second/second_module.rs"]
mod second_module;

#[path = "./third/third_module.rs"]
mod third_module;

#[path = "./fourth/fourth_module.rs"]
mod fourth_module;

// fn first_task() {
//     let result: Result<i32, io::Error> = first_module::jewelry_search();
//     match result {
//         Ok(count) => println!("{}", count),
//         Err(e) => println!("{}", e),
//     }
// }

// fn second_task() {
//     let (x, k, times) = second_module::read_input_data();
//     let result = second_module::alarm_clock(x, k, times);
//     println!("{}", result);
// }

fn third_task() {
    let (k, n) = third_module::read_input_data();
    let result: &str = third_module::cart_game(k, n);
    println!("{}", result);
}

fn fourth_task() {
    let input_line: String = input::read_input(
        "Введите количество коммитов и ответы системы тестирования (через пробел):",
    );
    // Разбиваем строку на отдельные токены по пробелам
    let tokens: Vec<&str> = input_line.split_whitespace().collect();
    let result: String = fourth_module::search_crash_commit(tokens);
    println!("{}", result);
}

fn main() {}
