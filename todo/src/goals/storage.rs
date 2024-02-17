use super::Goal;
use crate::{
    projects::ProjectsFile,
    utils::{FileSaver, Priority, RelationAction},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GoalsFile {
    pub objects: HashMap<i16, Goal>,
}

impl GoalsFile {
    pub fn add(
        &mut self,
        title: String,
        why: String,
        how: String,
        notes: String,
        priority: Priority,
        horizon: i8,
    ) -> i16 {
        let goal = Goal::new(
            self.get_latest_id(),
            title,
            why,
            how,
            notes,
            priority,
            horizon,
        );
        println!("New goal {} - {} created successfully", goal.id, goal.title);
        self.objects.entry(goal.id).or_insert(goal);
        self.save_changes();
        0
    }
    pub fn update(
        mut self,
        id: i16,
        title: Option<String>,
        why: Option<String>,
        how: Option<String>,
        notes: Option<String>,
        priority: Option<Priority>,
        horizon: Option<i8>,
    ) -> i16 {
        self.objects.entry(id).and_modify(|goal| {
            goal.update(title, why, how, notes, priority, horizon);
        });
        self.save_changes();
        0
    }

    pub async fn handle_relationships(from: i16, to: i16, action: &RelationAction) -> i16 {
        let mut objs = Self::get_or_create();
        let obj = objs.objects().get_mut(&to).unwrap();
        match &action {
            RelationAction::Add => obj.projects.insert(from),
            RelationAction::Remove => obj.projects.remove(&from),
        };
        Self::update_hours_per_week(obj).await;
        objs.save_changes();
        0
    }

    async fn update_hours_per_week(goal: &mut Goal) -> i16 {
        let projects = ProjectsFile::get_or_create().objects;
        //https://stackoverflow.com/questions/68344087/how-do-you-call-an-async-method-within-a-closure-like-within-map-in-rust
        goal.hours_per_week += goal
            .projects
            .iter()
            .map(|p| projects.get(p).unwrap().hours_per_week)
            .sum::<f64>();
        0
    }
}

impl FileSaver for GoalsFile {
    type ObjectStored = Goal;

    fn delete_by_title(&mut self, title: String) -> i16 {
        match self.objects.iter().find(|(_, t)| t.title == title) {
            Some((id, _)) => self.delete_by_id(*id),
            None => 1,
        }
    }

    fn objects(&mut self) -> &mut HashMap<i16, Goal> {
        &mut self.objects
    }
}
