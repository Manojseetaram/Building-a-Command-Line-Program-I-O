use chrono::NaiveDate;
use serde::{Deserialize, Serialize};


#[derive(Debug , Serialize , Deserialize , Clone)]
pub enum Priority {
    Low,
    Medium,
    High
}
#[derive(Debug , Serialize , Deserialize , Clone)]
pub struct Task{
    pub title : String,
    pub description : String,
    pub due_date : NaiveDate,
    pub priority : Priority,
    pub completed : bool
}
impl Task(){
    pub fn new(title : String , description : String , due_date : NaiveDate , priority : Priority )-> Self{
    Task{
       title ,
       description,
       due_date,
       priority,
       completed:false
    }

    }
    pub fn mark_completed(&mut self){
        self.completed = true;
    }
}