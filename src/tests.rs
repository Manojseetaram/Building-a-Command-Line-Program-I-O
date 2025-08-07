#[cfg(test)]
mod tests {
    use crate::models::{Priority, Task};

    
    use chrono::NaiveDate;


    #[test]
    fn test_task_creation(){
        let task = Task::new(
        "Test".to_string(),
        "Test task".to_string(),
        NaiveDate::from_ymd_opt(2025, 05, 19).unwrap(),
        Priority::Medium
        );
        assert_eq!(task.title , "Test");
        assert!(!task.completed)
    }
    #[test]
    fn test_task_completed(){
        let mut task = Task::new(
            "Test".to_string(),
            "Test task".to_string(),
            NaiveDate::from_ymd_opt(2025, 05, 19).unwrap(),
            Priority::Low
        );
        task.mark_complete();
        assert!(task.completed)
    }




}