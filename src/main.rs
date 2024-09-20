#[path = "first/first_module.rs"]
mod first_module;

#[path = "./utils/input.rs"]
mod input;

#[path = "./second/second_module.rs"]
mod second_module;

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

fn main() {}
