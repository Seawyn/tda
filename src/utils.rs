use chrono::{
    prelude::{NaiveDateTime, Local},
    TimeZone,
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::{
    fs,
    fmt,
    io::{Error, ErrorKind, BufReader},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub enum Status {
    /// Closed tasks
    Done,
    /// Ongoing tasks
    Todo,
    /// Tasks past deadline
    Overdue
}

/// Represents a single task
#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    /// The task's unique identifier
    id: i32,
    /// Task name
    task: String,
    /// Task name, Status enum (Done, Overdue, TODO)
    status: Status,
    /// Timestamp of creation
    timestamp: NaiveDateTime,
    deadline: Option<NaiveDateTime>
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

impl Entry {
    pub fn new(id: i32, name: String, deadline: Option<NaiveDateTime>) -> Self {
        Self {
            id: id,
            task: name,
            status: Status::Todo,
            timestamp: Local::now().naive_local(),
            deadline: deadline
        }
    }

    /// Check if task is past deadline based on current time
    pub fn is_overdue(&self) -> bool {
        let curr_time = Local::now().naive_local();
        match self.deadline {
            Some(d) => d < curr_time,
            _ => false
        }
    }

    /// Get number of days since the task has been created
    pub fn delta() {
        unimplemented!()
    }
}

/// Task list
#[derive(Serialize, Deserialize)]
pub struct List {
    /// Vector containing all tasks
    pub entries: Vec<Entry>,
    /// Current id cursor
    id_tracker: i32
}

impl List {
    /// Constructor
    pub fn new() -> Self {
        Self { entries: Vec::new(), id_tracker: 0 }
    }

    /// Return total tasks
    pub fn get_size(&self) -> usize {
        self.entries.len()
    }

    /// Return all tasks
    pub fn get_all(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn get_cursor(&self) -> i32 {
        self.id_tracker
    }

    pub fn inc_cursor(&mut self) {
        self.id_tracker += 1;
    }

    pub fn add_task(&mut self, task: &str, deadline: Option<NaiveDateTime>){
        if task == "" { 
            println!("Cannot add empty task name");
        }
    
        let new_task = Entry::new(self.get_cursor(), task.to_string(), deadline);
        self.entries.push(new_task);
        self.inc_cursor();
    }

    pub fn close_task(&mut self, id: i32) -> Result<(), Error> {
        for i in 0..self.get_size() {
            if self.entries[i].id == id && self.entries[i].status != Status::Done{
                self.entries[i].status = Status::Done;
                return Ok(())
            }
        }
        Err(Error::new(ErrorKind::InvalidInput, format!("Open task with id {} not found", id)))
    }

    /// Obtain count of tasks by status
    pub fn get_status(&self) -> HashMap<Status, u8> {
        let mut counts = HashMap::from([
            (Status::Todo, 0),
            (Status::Done, 0),
            (Status::Overdue, 0)
        ]);

        for el in self.get_all().iter() {
            let val = counts.get(&el.status).unwrap();
            counts.insert(el.status.clone(), val + 1);
        }

        counts
    }

    pub fn check_overdues(&mut self) {
        for i in 0..self.get_size() {
            if self.entries[i].is_overdue() {
                self.entries[i].status = Status::Overdue;
            }
        }
    }
}

/// Open JSON file
pub fn open_file(fpath: &str) -> List {
    let content = fs::File::open(fpath).unwrap();
    let reader = BufReader::new(content);

    let res = serde_json::from_reader(reader).unwrap();
    res
}

/// Reads JSON file or creates a new task list if there is no file
pub fn read_or_create(fpath: &str) -> List{
    if Path::new(fpath).exists() {
        open_file(fpath)
    }
    else {
        List::new()
    }
}

/// Save task list to JSON file
pub fn export(list: List, fpath: &str) {
    let f = serde_json::to_string(&list).unwrap();

    fs::write(fpath, f).expect("Error writing file");
}

pub fn parse_deadline(mut deadline_raw: String) -> Option<NaiveDateTime> {
    if deadline_raw.ends_with("\n") {
        deadline_raw.pop();
    }

    let parts = deadline_raw.split("-").collect::<Vec<&str>>();

    if parts.len() != 3 {
        return None
    }

    let year = parts[0].to_string().parse::<i32>().ok();
    let month = parts[1].to_string().parse::<u32>().ok();
    let day = parts[2].to_string().parse::<u32>().ok();

    let new_local;
    match (year, month, day) {
        (Some(year), Some(month), Some(day)) => new_local = Local.with_ymd_and_hms(year, month, day, 0, 0, 0),
        _ => return None
    };

    match new_local.single() {
        Some(t) => Some(t.naive_local()),
        _ => None
    }
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::Datelike;

    use super::*;
    use crate::utils::List;

    #[test]
    fn list_size() {
        let mut list = List::new();
        let total_tasks = 100;
        for i in 0..total_tasks {
            let curr_task_name = format!("Sample task {}", i);
            list.add_task(&curr_task_name, None);
        }
        assert_eq!(list.get_size(), total_tasks);
    }

    #[test]
    fn new_task() {
        let mut list = List::new();
        list.add_task("Sample task", None);

        let to_close: i32 = 0;

        assert_eq!(list.entries[to_close as usize].status, Status::Todo);
        list.close_task(to_close).unwrap();
        assert_eq!(list.entries[to_close as usize].status, Status::Done);
    }

    #[test]
    fn parse_pass() {
        let line = String::from("2024-01-01\n");
        let res = parse_deadline(line).unwrap();
        let exp = Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap().naive_local();

        assert_eq!(res, exp);
    }

    #[test]
    fn parse_fail_not_date() {
        let line_error = String::from("2024-01-011232");
        let res = parse_deadline(line_error);
        assert!(res.is_none());
    }

    #[test]
    fn parse_fail_empty() {
        let line_error = String::from("");
        let res = parse_deadline(line_error);
        assert!(res.is_none());
    }

    #[test]
    fn overdue_true() {
        let deadline = parse_deadline(String::from("2000-01-01"));
        let entry = Entry::new(0, String::from("test entry"), deadline);

        assert!(entry.is_overdue());
    }

    #[test]
    fn overdue_missing() {
        // Empty deadline, results in null
        let deadline = parse_deadline(String::from(""));
        let entry = Entry::new(0, String::from("test entry"), deadline);

        assert!(!entry.is_overdue());
    }

    #[test]
    fn not_overdue() {
        // One day from now
        let curr_time = Local::now().naive_local() + Duration::from_secs(60*60*24);
        let deadline_str = format!("{}-{}-{}", curr_time.year(), curr_time.month(), curr_time.day());
        let deadline = parse_deadline(String::from(deadline_str));
        let entry = Entry::new(0, String::from("test entry"), deadline);

        assert!(!entry.is_overdue());
    }
}
