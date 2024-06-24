use std::collections::HashMap;
use std::fs;
use std::path::Path;

mod project_data;
mod toml_utils;

use project_data::{InRepoDB, Project, Ticket};

const BASE_DIR: &str = "example_data";
const PROJECTS_DIR: &str = "projects";
const TICKETS_DIR: &str = "tickets";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let in_repo_db = collect_in_repo_db()?;

    // Use the deserialized data (example)

    let tiks: Vec<_> = in_repo_db
        .iter_tickets()
        .filter(|t| t.get_project_id() == "P0001")
        .filter(|t| t.is_open())
        .collect();
    println!("{:#?}", tiks);

    Ok(())
}

fn collect_in_repo_db() -> Result<InRepoDB, Box<dyn std::error::Error>> {
    let base_path = Path::new(BASE_DIR);
    let project_path = base_path.join(PROJECTS_DIR);
    let ticket_path = base_path.join(TICKETS_DIR);
    let project_files = toml_utils::get_toml_files_in_dir(&project_path)?;
    let ticket_files = toml_utils::get_toml_files_in_dir(&ticket_path)?;
    let mut projects = HashMap::new();
    for proj_file in project_files {
        let proj_contents = fs::read_to_string(&proj_file)?;
        let project: Project = toml::from_str(&proj_contents)?;
        projects.insert(project.id.clone(), project);
    }
    let mut tickets = HashMap::new();
    for ticket_file in ticket_files {
        let ticket_contents = fs::read_to_string(&ticket_file)?;
        let ticket: Ticket = toml::from_str(&ticket_contents)?;
        tickets.insert(ticket.id.clone(), ticket);
    }
    let in_repo_db = InRepoDB::new(projects, tickets);
    Ok(in_repo_db)
}
