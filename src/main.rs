pub mod utils;
use std::io;

const FILENAME: &str = "tasks.json";

fn main() {
    let mut all_tasks = utils::read_or_create(FILENAME);
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
            
                println!("Add deadline? (format: YYYY-MM-DD)");
                let mut deadline_resp = String::new();
                io::stdin().read_line(&mut deadline_resp).expect("Error reading input");
                
                let deadline = utils::parse_deadline(deadline_resp);

                all_tasks.add_task(task_name, deadline);
            },
            "help" => utils::show_help(),
            "list" => {
                all_tasks.check_overdues();
                utils::list_tasks(&all_tasks)
            },
            "close" => {
                let task_id = input
                    .strip_prefix(instr).unwrap_or("")
                    .trim_start().trim().parse::<i32>().unwrap();
                all_tasks.close_task(task_id).unwrap();
            },
            "remove" => println!("TODO: Remove task"),
            "quit" => break,
            "" => (),
            _ => println!("Unknown command")
        };

        input.clear();
    }

    utils::export(all_tasks, FILENAME);
}
