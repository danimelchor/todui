use super::formats::Format;
use crate::{configuration::Settings, task::Task};

pub fn print_task(task: &Task, format: Option<Format>, settings: &Settings) {
    match format {
        Some(Format::Json) => println!("{}", serde_json::to_string_pretty(&task).unwrap()),
        _ => {
            println!("{} {}", task.id.unwrap(), task.name);
            println!("Date: {}", task.date);
            println!("Repeats: {:}", task.repeats);
            if let Some(description) = &task.description {
                println!("Description: {}", description);
            }
            println!("Completed: {}", settings.icons.get_icon(task.completed));
        }
    }
}

pub fn print_tasks(tasks: Vec<&Task>, format: Option<Format>, settings: &Settings) {
    match format {
        Some(Format::Json) => println!("{}", serde_json::to_string_pretty(&tasks).unwrap()),
        _ => {
            for task in tasks {
                let id = task.id.unwrap();
                let name = &task.name;
                let date = task.date;
                let repeats = &task.repeats;
                let completed = task.completed;

                let x = settings.icons.get_icon(completed);
                println!("{} {} ({}) {} {}", x, name, id, date, repeats);
            }
        }
    }
}
