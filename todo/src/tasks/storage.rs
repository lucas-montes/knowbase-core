use super::Task;
use crate::utils::{Day, FileSaver, Priority};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TasksFile {
    pub objects: HashMap<i16, Task>,
    pub seen: HashSet<i16>,
    pub last_check: String,
}

impl TasksFile {
    pub fn add(
        &mut self,
        title: String,
        description: String,
        start: String,
        end: String,
        priority: Priority,
        after: Option<i16>,
        days: Vec<Day>,
    ) -> i16 {
        let task: Task = Task::new(
            self.get_latest_id(),
            title,
            description,
            start,
            end,
            priority,
            after,
            days,
        );
        println!("New Task {} - {} created successfully", task.id, task.title);
        self.objects.entry(task.id).or_insert(task);
        self.save_changes();
        0
    }
    pub fn update(
        mut self,
        id: i16,
        title: Option<String>,
        description: Option<String>,
        start: Option<String>,
        end: Option<String>,
        priority: Option<Priority>,
        after: Option<String>,
        done: Option<bool>,
        days: Option<Vec<Day>>,
    ) -> i16 {
        self.objects.entry(id).and_modify(|todo| {
            todo.update(title, description, start, end, priority, after, done, days);
        });

        self.save_changes();
        0
    }
}

impl FileSaver for TasksFile {
    type ObjectStored = Task;

    fn delete_by_title(&mut self, title: String) -> i16 {
        match self.objects.iter().find(|(_, t)| t.title == title) {
            Some((id, _)) => self.delete_by_id(*id),
            None => 1,
        }
    }

    fn objects(&mut self) -> &mut HashMap<i16, Task> {
        &mut self.objects
    }
}
