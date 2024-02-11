use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;
use std::io::BufReader;
use std::path::Path;
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub enum Status {
    Done,
    Todo,
    Overdue
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    id: i32,
    task: String,
    status: Status
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let marker = match self.status {
            Status::Done => "-",
            Status::Overdue => "*",
            Status::Todo => "|"
        };
        write!(f, "{} {} {}", marker, self.id, self.task)
    }
}

#[derive(Serialize, Deserialize)]
pub struct List {
    pub entries: Vec<Entry>,
    pub id_tracker: i32
}

impl List {
    pub fn get_size(&self) -> usize {
        self.entries.len()
    }

    pub fn get_all(&self) -> &Vec<Entry> {
        &self.entries
    }
}

pub fn open_file(fpath: &str) -> List {
    let content = fs::File::open(fpath).unwrap();
    let reader = BufReader::new(content);

    let res = serde_json::from_reader(reader).unwrap();
    res
}

pub fn read_or_create(fpath: &str) -> List{
    if Path::new(fpath).exists() {
        open_file(fpath)
    }
    else {
        List{entries: Vec::new(), id_tracker: 0}
    }
}

pub fn export(list: List, fpath: &str) {
    let f = serde_json::to_string(&list).unwrap();

    fs::write(fpath, f).expect("Error writing file");
}

pub fn get_status(list: &List) -> HashMap<Status, u8> {
    let mut counts = HashMap::from([
        (Status::Todo, 0),
        (Status::Done, 0),
        (Status::Overdue, 0)
    ]);

    for el in list.entries.iter() {
        let val = counts.get(&el.status).unwrap();
        counts.insert(el.status.clone(), val + 1);
    }

    counts
}

pub fn add_task(mut list: List, task: &str) -> List{
    if task == "" { 
        println!("Cannot add empty task name");
        return list
    }

    let new_task = Entry{
        id: list.id_tracker,
        task: task.to_string(), status: Status::Todo
    };
    list.entries.push(new_task);
    list.id_tracker += 1;
    list
}

pub fn update_task(mut list: List, id: i32) -> Result<List, Error> {
    for i in 0..list.entries.len() {
        if list.entries[i].id == id { 
            list.entries[i].status = Status::Done;
            return Ok(list)
        }
    }
    Err(Error::new(ErrorKind::InvalidInput, "Task not found"))
}

pub fn list_tasks(list: &List) {
    let mut overdues: Vec<Entry> = Vec::new();
    let mut todos: Vec<Entry> = Vec::new();
    let mut dones: Vec<Entry> = Vec::new();

    for i in 0..list.entries.len() {
        match list.entries[i].status {
            Status::Done => dones.push(list.entries[i].clone()),
            Status::Overdue => overdues.push(list.entries[i].clone()),
            Status::Todo => todos.push(list.entries[i].clone())
        }
    }

    if overdues.len() == 0 {
        println!("You have no overdue tasks");
    }
    else {
        for el in overdues { print!("{:?}", el) }
    }

    if todos.len() == 0 {
        println!("You have no tasks")
    }
    else {
        for el in todos { print!("{:?}", el) }
    }

    if dones.len() > 0 { 
        for el in dones {print!("{:?}", el) }
    }
}

pub fn close_task(mut list: List, id: i32) -> Result<List, Error> {
    for i in 0..list.entries.len() {
        if list.entries[i].id == id && list.entries[i].status != Status::Done {
            list.entries[i].status = Status::Done;
            return Ok(list)
        }
    }
    Err(Error::new(ErrorKind::InvalidInput, "Open task with provided id not found"))
}

pub fn show_help() {
    let help_string = "
    Usage:
    add [task_name]
        Adds new task named [task_name] under TODO.
    
    list
        List all overdue, todo and closed tasks, in that order.
    
    close [task_id]
        Close task with provided [task_id], moves it from TODO to done.
    
    remove [task_id]
        Removes task from list. Other task ids are not affected.

    quit
        Exit TODO cli.
    ";
    println!("{}", help_string);
}

#[test]
fn test_update_task() {
    let mut list = List{entries: Vec::new(), id_tracker: 0};
    list = add_task(list, "Sample task");

    let to_close: i32 = 0;

    assert_eq!(&list.entries[to_close], Status::Todo);
    list = update_task(list, to_close).unwrap();
    assert_eq!(list.entries[to_close], Status::Done);
}