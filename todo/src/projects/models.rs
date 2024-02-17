use std::collections::HashSet;

use crate::utils::{Priority};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i16,
    pub title: String,
    pub description: String,
    pub start: String,
    pub end: String,
    pub notes: String,
    pub priority: Priority,
    pub accomplished: bool,
    pub tasks: HashSet<i16>,
    pub hours_per_week: f64,
}

impl Project {
    pub fn new(
        id: i16,
        title: String,
        description: String,
        start: String,
        end: String,
        notes: String,
        priority: Priority,
    ) -> Self {
        Project {
            id,
            title,
            description,
            start,
            end,
            notes,
            priority,
            accomplished: false,
            tasks: HashSet::new(),
            hours_per_week: 0.0,
        }
    }

    pub fn update<'a>(
        &'a mut self,
        title: Option<String>,
        description: Option<String>,
        start: Option<String>,
        end: Option<String>,
        notes: Option<String>,
        priority: Option<Priority>,
        accomplished: Option<bool>,
    ) -> &'a mut Self {
        self.title = title.unwrap_or(self.title.clone());
        self.description = description.unwrap_or(self.description.clone());
        self.start = start.unwrap_or(self.start.clone());
        self.end = end.unwrap_or(self.end.clone());
        self.notes = notes.unwrap_or(self.notes.clone());
        self.priority = priority.unwrap_or(self.priority.clone());
        self.accomplished = accomplished.unwrap_or(self.accomplished.clone());
        self
    }

    pub fn in_stand_by(&self) -> bool {
        self.start.is_empty()
    }
}
