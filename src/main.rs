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
            "help" => println!("TODO: Show guide"),
            "list" => utils::list_tasks(&all_tasks),
            "close" => println!("TODO: Close task"),
            "remove" => println!("TODO: Remove task"),
            "quit" => break,
            "" => (),
            _ => println!("Unknown command")
        };

        input.clear();
    }

    utils::export(all_tasks, "example.json");
    
    /*
    all_tasks = utils::add_task(all_tasks, "Sample task");

    let temp = utils::get_status(&all_tasks);
    println!("{:?}", temp);

    utils::list_tasks(&all_tasks);

    utils::export(all_tasks, "example.json");
     */
}
