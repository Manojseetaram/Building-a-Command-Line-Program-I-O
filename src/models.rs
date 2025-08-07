use chrono::NaiveDate;

pub enum Priority {
    Low,
    Medium,
    High
}
pub struct Task{
    pub title : String,
    pub description : String,
    pub due_date : NaiveDate,
    pub priority : Priority,
    pub completed : bool
}