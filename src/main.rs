use crate::storage::{load_tasks, save_tasks};

mod models;
mod storage;
fn main() {
    println!("Hello, world!");
    let tasks = load_tasks().unwrap_or_else(|_| Vec::new());
    loop {
        println!("\n Task manager");
        println!("1. Add Task");
        println!("2. View Task");
        println!("3. Mark compoleted");
        println!("4. Save and exits");
        print!("Choose : ");


        let choice = String::new();
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
