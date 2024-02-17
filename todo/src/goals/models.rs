use crate::utils::Priority;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: i16,
    pub title: String,
    pub why: String,
    pub how: String,
    pub notes: String,
    pub priority: Priority,
    pub projects: HashSet<i16>,
    pub hours_per_week: f64,
    pub horizon: i8,
    pub created_at: String,
}

impl Goal {
    pub fn new(
        id: i16,
        title: String,
        why: String,
        how: String,
        notes: String,
        priority: Priority,
        horizon: i8,
    ) -> Self {
        Goal {
            id,
            title,
            why,
            how,
            notes,
            priority,
            horizon,
            projects: HashSet::new(),
            hours_per_week: 0.0,
            created_at: Local::now().naive_local().to_string(),
        }
    }

    pub fn update<'a>(
        &'a mut self,
        title: Option<String>,
        why: Option<String>,
        how: Option<String>,
        notes: Option<String>,
        priority: Option<Priority>,
        horizon: Option<i8>,
    ) -> &'a mut Self {
        self.title = title.unwrap_or(self.title.clone());
        self.why = why.unwrap_or(self.why.clone());
        self.how = how.unwrap_or(self.how.clone());
        self.notes = notes.unwrap_or(self.notes.clone());
        self.priority = priority.unwrap_or(self.priority);
        self.horizon = horizon.unwrap_or(self.horizon.clone());
        self
    }
}
