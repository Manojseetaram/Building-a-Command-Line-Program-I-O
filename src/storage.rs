use std::{fs::File, io::{self, BufReader, Write}};
use  crate::models::Task;


const FILE_PATH : &str= "task.json";
pub fn save_tasks(tasks : &Vec<Task>)-> io::Result<()>{
    let json = serde_json::to_string_pretty(&tasks)?;
    let mut file = File::create(FILE_PATH)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
pub fn load_tasks()-> io::Result<Vec<Task>>{
   match File::open(FILE_PATH) {
       Ok(file)=>{
           let  reader = BufReader::new(file) ;
           let tasks :Vec<Task> = serde_json::from_reader(reader)?;
           Ok(tasks)

       }
       Err(_) => Ok(Vec::new())
   }
}