use super::TasksFile;
use crate::utils::{ColorWhen, Day,FileSaver, Priority};

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TaskArgs {
    #[command(subcommand)]
    pub command: TaskCommands,
    #[arg(
        long,
        require_equals = true,
        value_name = "WHEN",
        num_args = 0..=1,
        default_value_t = ColorWhen::Auto,
        default_missing_value = "always",
        value_enum
    )]
    color: ColorWhen,
}

#[derive(Debug, Subcommand)]
pub enum TaskCommands {
    /// Create a new object
    #[command(arg_required_else_help = true)]
    Create(CreateTask),

    /// Update an actual object
    #[command(arg_required_else_help = true)]
    Update(UpdateTask),

    /// Delete one object
    #[command(arg_required_else_help = true)]
    Delete(DeleteTask),

    /// Read one or more objects
    #[command(arg_required_else_help = true)]
    Read(ReadTask),
}

impl TaskCommands {
    pub fn handle_commands(&self) -> i16 {
        match self {
            TaskCommands::Create(args) => args.run(),
            TaskCommands::Update(args) => args.run(),
            TaskCommands::Delete(args) => args.run(),
            TaskCommands::Read(args) => args.run(),
        };
        0
    }
}

#[derive(Debug, Args, Clone)]
pub struct CreateTask {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    description: Option<String>,
    #[arg(short, long, value_name = "Start date")]
    start: Option<String>,
    #[arg(short, long, value_name = "End date")]
    end: Option<String>,
    #[arg(
        short,
        long,
        default_value_t = Priority::Low,
        default_missing_value = "Low",
        value_enum, 
        required = false
    )]
    priority: Priority,
    #[arg(short, long, default_value_t = true, required = false, help="Is a task meant to be done only once")]
    one_off: bool,
    #[arg(long, num_args = 0..=7, value_enum, required = false)]
    days: Option<Vec<Day>>,
    #[arg(short, long)]
    after: Option<i16>,
}

impl CreateTask {
    fn run(&self) -> i16 {
        let mut todos_file = TasksFile::get_or_create();
        todos_file.add(
            self.title.clone(),
            self.description.clone().unwrap_or(String::from("")),
            self.start.clone().unwrap_or(String::from("")),
            self.end.clone().unwrap_or(String::from("")),
            self.priority,
            self.after,
            self.days.clone().unwrap_or(vec![]),
        )
    }
}

#[derive(Debug, Args, Clone)]
pub struct ReadTask {
    #[arg(short, long, default_value_t = true, required=false)]
    all: bool,
    #[arg(short, long, required=false)]
    id: Option<i16>,
    #[arg(short, long, required=false)]
    title: Option<String>,
    #[arg(short, long, required=false)]
    start: Option<String>,
    #[arg(short, long, required=false)]
    end: Option<String>,
    #[arg(short, long, value_enum, required=false)]
    priority: Option<Priority>,
    #[arg(long, num_args = 0..=7, value_enum, required=false)]
    days: Option<Vec<Day>>,
    #[arg(short, long, required=false)]
    done: Option<bool>,
}

impl ReadTask {
    fn run(&self) -> i16 {
        let mut todos_file = TasksFile::get_or_create();
        let tasks = todos_file.objects();
        let tasks_to_show: std::collections::HashMap<i16, super::Task> = if let Some(v) = self.done {
            tasks
                .iter()
                .filter(|&(_, task)| task.done == v)
                .map(|(&k, v)| (k, v.clone()))
                .collect()
        } else {
            tasks.clone()
        };
        todos_file.read(&tasks_to_show);
        0
    }
}

#[derive(Debug, Args, Clone)]
pub struct UpdateTask {
    #[arg(short, long)]
    id: i16,
    #[arg(short, long)]
    title: Option<String>,
    #[arg(short, long)]
    description: Option<String>,
    #[arg(short, long)]
    start: Option<String>,
    #[arg(short, long)]
    end: Option<String>,
    #[arg(short, long, value_enum)]
    priority: Option<Priority>,
    #[arg(short, long)]
    done: Option<bool>,
    #[arg(long, num_args = 0..=7, value_enum)]
    days: Option<Vec<Day>>,
    #[arg(short, long)]
    after: Option<String>,
}

impl UpdateTask {
    
    fn run(&self) -> i16 {
        let todos_file = TasksFile::get_or_create();
        todos_file.update(
            self.id,
            self.title.clone(),
            self.description.clone(),
            self.start.clone(),
            self.end.clone(),
            self.priority,
            self.after.clone(),
            self.done,
            self.days.clone(),
        )
    }
}

#[derive(Debug, Args, Clone)]
pub struct DeleteTask {
    #[arg(short, long)]
    id: Option<i16>,
    #[arg(short, long)]
    title: Option<String>,
}

impl DeleteTask {
    fn run(&self) -> i16 {
        let mut todos_file = TasksFile::get_or_create();
        todos_file.delete(self.id, self.title.clone())
    }
}
