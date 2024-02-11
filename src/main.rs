pub mod utils;
use std::io;

fn main() {
    let mut all_tasks = utils::read_or_create("example.json");

    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input).expect("Error reading input");

        let instr =  input
            .split_whitespace()
            .next()
            .unwrap_or("");
        
        match instr {
            "add" => {
                let task_name = input
                    .strip_prefix(instr).unwrap_or("")
                    .trim_start();
            
                all_tasks = utils::add_task(all_tasks, task_name);
            },
            "help" => utils::show_help(),
            "list" => utils::list_tasks(&all_tasks),
            "close" => {
                let task_id = input
                    .strip_prefix(instr).unwrap_or("")
                    .trim_start().trim().parse::<i32>().unwrap();
                all_tasks = utils::close_task(all_tasks, task_id).unwrap();
            },
            "remove" => println!("TODO: Remove task"),
            "quit" => break,
            "" => (),
            _ => println!("Unknown command")
        };

        input.clear();
    }

    utils::export(all_tasks, "example.json");
}
