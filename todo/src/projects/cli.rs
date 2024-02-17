use super::ProjectsFile;
use crate::utils::{ColorWhen, FileSaver, Priority};

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: ProjectCommands,
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
pub enum ProjectCommands {
    /// Create a new object
    #[command(arg_required_else_help = true)]
    Create(CreateProject),

    /// Update an actual object
    #[command(arg_required_else_help = true)]
    Update(UpdateProject),

    /// Delete one object
    #[command(arg_required_else_help = true)]
    Delete(DeleteProject),

    /// Read one or more objects
    #[command(arg_required_else_help = true)]
    Read(ReadProject),
}

impl ProjectCommands {
    pub fn handle_commands(&self) -> i16 {
        match self {
            ProjectCommands::Create(args) => args.run(),
            ProjectCommands::Update(args) => args.run(),
            ProjectCommands::Delete(args) => args.run(),
            ProjectCommands::Read(args) => args.run(),
        };
        0
    }
}

#[derive(Debug, Args, Clone)]
pub struct CreateProject {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    description: Option<String>,
    #[arg(short, long)]
    start: Option<String>,
    #[arg(short, long)]
    end: Option<String>,
    #[arg(short, long)]
    notes: Option<String>,
    #[arg(
        short,
        long,
        require_equals = true,
        default_value_t = Priority::Low,
        default_missing_value = "Low",
        value_enum, 
        required = false
    )]
    priority: Priority,
}

impl CreateProject {
    fn run(&self) -> i16 {
        let mut projects_file = ProjectsFile::get_or_create();
        projects_file.add(
            self.title.clone(),
            self.description.clone().unwrap_or(String::from("")),
            self.start.clone().unwrap_or(String::from("")),
            self.end.clone().unwrap_or(String::from("")),
            self.notes.clone().unwrap_or(String::from("")),
            self.priority,
        )
    }
}

#[derive(Debug, Args, Clone)]
pub struct DeleteProject {
    #[arg(short, long)]
    id: Option<i16>,
    #[arg(short, long)]
    title: Option<String>,
}

impl DeleteProject {
    fn run(&self) -> i16 {
        let mut projects_file = ProjectsFile::get_or_create();
        projects_file.delete(self.id, self.title.clone())
    }
}

#[derive(Debug, Args, Clone)]
pub struct ReadProject {
    #[arg(short, long, default_value_t = true, required = false)]
    all: bool,
    #[arg(short, long, required = false)]
    id: Option<i16>,
    #[arg(short, long, required = false)]
    title: Option<String>,
    #[arg(short, long, value_enum, required = false)]
    priority: Option<Priority>,
}

impl ReadProject {
    fn run(&self) -> i16 {
        let mut projects_file = ProjectsFile::get_or_create();
        let objs = projects_file.objects().clone();
        projects_file.read(&objs);
        0
    }
}

#[derive(Debug, Args, Clone)]
pub struct UpdateProject {
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
    #[arg(short, long)]
    notes: Option<String>,
    #[arg(short, long, value_enum)]
    priority: Option<Priority>,
    #[arg(short, long)]
    accomplished: Option<bool>,
}

impl UpdateProject {
    fn run(&self) -> i16 {
        let projects_file = ProjectsFile::get_or_create();
        projects_file.update(
            self.id,
            self.title.clone(),
            self.description.clone(),
            self.start.clone(),
            self.end.clone(),
            self.notes.clone(),
            self.priority,
            self.accomplished.clone(),
        )
    }
}
