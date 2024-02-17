use crate::utils::notify;
use crate::utils::{Day, Priority};

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i16,
    pub title: String,
    pub description: String,
    pub start: String,
    pub end: String,
    pub priority: Priority,
    pub done: bool,
    pub days: Vec<Day>,
    pub after: Option<i16>,
}

impl Task {
    pub fn new(
        id: i16,
        title: String,
        description: String,
        start: String,
        end: String,
        priority: Priority,
        after: Option<i16>,
        days: Vec<Day>,
    ) -> Self {
        Task {
            id,
            title,
            description,
            start,
            end,
            priority,
            after,
            done: false,
            days,
        }
    }

    pub fn update<'a>(
        &'a mut self,
        title: Option<String>,
        description: Option<String>,
        start: Option<String>,
        end: Option<String>,
        priority: Option<Priority>,
        after: Option<String>,
        done: Option<bool>,
        days: Option<Vec<Day>>,
    ) -> &'a mut Self {
        let after = match after {
            Some(s) => s.parse::<i16>().ok(),
            None => self.after,
        };

        self.title = title.unwrap_or(self.title.clone());
        self.description = description.unwrap_or(self.description.clone());
        self.start = start.unwrap_or(self.start.clone());
        self.end = end.unwrap_or(self.end.clone());
        self.priority = priority.unwrap_or(self.priority.clone());
        self.after = after;
        self.done = done.unwrap_or(self.done.clone());
        self.days = days.unwrap_or(self.days.clone());
        self
    }

    pub fn duration(&self) -> f64 {
        let m = match self.days.len() {
            0 => 1.0,
            _ => self.days.len() as f64,
        };

        self.duration_in_hours() * 24.0 * m
    }

    pub fn is_one_off(&self) -> bool {
        self.days.is_empty()
    }

    fn duration_in_hours(&self) -> f64 {
        let start_date = match NaiveTime::parse_from_str(&self.start, "%H:%M") {
            Ok(value) => value,
            Err(err) => panic!("oupsi, {:?}", err),
        };
        let end_date = match NaiveTime::parse_from_str(&self.end, "%H:%M") {
            Ok(value) => value,
            Err(err) => panic!("oupsi, {:?}", err),
        };
        let duration = end_date.signed_duration_since(start_date);
        duration.num_hours() as f64 + (duration.num_minutes() as f64 / 60.0)
    }

    pub async fn to_notification(&self) {
        let due_date: &str = if self.start.is_empty() {
            &self.end
        } else {
            &self.start
        };
        let summary = format!("The task {} is due {due_date}", &self.title);
        notify(&summary, &self.description, self.priority.to_dialog_alarm()).await;
    }
}
