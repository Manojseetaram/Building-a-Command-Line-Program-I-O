use chrono::NaiveDate;

use crate::{models::{Priority, Task}, storage::{load_tasks, save_tasks}};

use std::io::{self , Write};
mod models;
mod storage;
mod tests;
fn main() {
    println!("Hello, world!");
    let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());
    loop {
        println!("\n Task manager");
        println!("1. Add Task");
        println!("2. View Task");
        println!("3. Mark compoleted");
        println!("4. Save and exits");
        print!("Choose : ");


        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim(){
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => {
                save_tasks(&tasks).unwrap();
                println!("Task saved"); 
                break;
            },
            _=> println!("Invalid Choice")
        }
    }
}
fn add_task(tasks: &mut Vec<Task>){
   let mut input = String::new();

   print!("Title : ");
   io::stdout().flush().unwrap();
   io::stdin().read_line(&mut input).unwrap();
   let title = input.trim().to_string();
   input.clear();

   print!("Description : ");
   io::stdout().flush().unwrap();
   io::stdin().read_line(&mut input).unwrap();
   let description =input.trim().to_string();
   input.clear();

   print!("Due Date (YYYY--MM--DD): ");
   io::stdout().flush().unwrap();
   io::stdin().read_line(&mut input).unwrap();
   let due_date = NaiveDate::parse_from_str(&input.trim(), "%Y-%m-%d").unwrap();
   input.clear();

   print!("Priority (Low/Medium/High) : ");
   io::stdout().flush().unwrap();
   io::stdin().read_line(&mut input).unwrap();
   let priority = match input.trim(){
    "High" => Priority::High,
    "Medium" =>Priority::Medium,
    _=>Priority::Low
   };
   let task = Task::new(title, description, due_date, priority);
    tasks.push(task);
   }
    

fn view_tasks(tasks: &Vec<Task>){
  println!("\n Your Tasks : ");
  for (i , task) in  tasks.iter().enumerate(){
      println!(
        "{}. [{}] {} - Due: {} ({:?})",
         i + 1,
            if task.completed { "✔" } else { " " },
            task.title,
            task.due_date,
            task.priority
      )
  }
    
}
fn mark_task_completed(tasks: &mut Vec<Task>){
     view_tasks(tasks);
    print!("Enter task number to mark complete: ");
    io::stdout().flush().unwrap();
    let mut index = String::new();
    io::stdin().read_line(&mut index).unwrap();

    if let Ok(i) = index.trim().parse::<usize>() {
        if let Some(task) = tasks.get_mut(i - 1) {
            task.mark_complete();
            println!("Task marked complete.");
        } else {
            println!("Invalid task number.");
        }
    }
}
