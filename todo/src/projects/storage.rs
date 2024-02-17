use super::Project;
use crate::{
    tasks::TasksFile,
    utils::{FileSaver, Priority, RelationAction},
};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsFile {
    pub objects: HashMap<i16, Project>,
}

impl ProjectsFile {
    pub fn add(
        &mut self,
        title: String,
        description: String,
        start: String,
        end: String,
        notes: String,
        priority: Priority,
    ) -> i16 {
        let start = if start == "now" {
            Local::now().naive_local().to_string()
        } else {
            start.to_owned()
        };

        let project: Project = Project::new(
            self.get_latest_id(),
            title,
            description,
            start,
            end,
            notes,
            priority,
        );
        println!(
            "New project {} - {} created successfully",
            project.id, project.title
        );
        self.objects.entry(project.id).or_insert(project);
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
        notes: Option<String>,
        priority: Option<Priority>,
        accomplished: Option<bool>,
    ) -> i16 {
        self.objects.entry(id).and_modify(|todo| {
            todo.update(
                title,
                description,
                start,
                end,
                notes,
                priority,
                accomplished,
            );
        });

        self.save_changes();
        0
    }

    pub async fn handle_relationships(from: i16, to: i16, action: &RelationAction) -> i16 {
        let mut objs = Self::get_or_create();
        let project = objs.objects().get_mut(&to).unwrap();
        match &action {
            RelationAction::Add => project.tasks.insert(from),
            RelationAction::Remove => project.tasks.remove(&from),
        };
        Self::update_hours_per_week(project).await;
        objs.save_changes();
        0
    }

    async fn update_hours_per_week(project: &mut Project) -> i16 {
        let tasks = TasksFile::get_or_create().objects;
        project.hours_per_week += project
            .tasks
            .iter()
            .map(|t| tasks.get(t).unwrap().duration())
            .sum::<f64>();
        0
    }
}

impl FileSaver for ProjectsFile {
    type ObjectStored = Project;

    fn delete_by_title(&mut self, title: String) -> i16 {
        match self.objects.iter().find(|(_, t)| t.title == title) {
            Some((id, _)) => self.delete_by_id(*id),
            None => 1,
        }
    }

    fn objects(&mut self) -> &mut HashMap<i16, Project> {
        &mut self.objects
    }
}
