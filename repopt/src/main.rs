use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

mod project_data;
mod toml_utils;

use project_data::{InRepoDB, Project, Ticket};

const BASE_DIR: &str = "example_data";
const PROJECTS_DIR: &str = "projects";
const TICKETS_DIR: &str = "tickets";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let in_repo_db = collect_in_repo_db()?;

    // Use the deserialized data (example)

    println!("Project one Open Tickets:");
    let tiks: Vec<_> = in_repo_db
        .iter_tickets()
        .filter(|t| t.get_project_id() == "P0001")
        .filter(|t| t.is_open())
        .collect();
    println!("{:#?}", tiks);

    println!("Project two Closed Tickets:");
    let tiks: Vec<_> = in_repo_db
        .iter_tickets()
        .filter(|t| t.get_project_id() == "P0002")
        .filter(|t| !t.is_open())
        .collect();
    println!("{:#?}", tiks);

    Ok(())
}

fn collect_in_repo_db() -> Result<InRepoDB, Box<dyn std::error::Error>> {
    let project_path = PathBuf::from(BASE_DIR).join(PROJECTS_DIR);
    let ticket_path = PathBuf::from(BASE_DIR).join(TICKETS_DIR);

    let projects: HashMap<String, Project> = toml_utils::get_toml_files_in_dir(&project_path)?
        .into_iter()
        .map(|proj_file| -> Result<_, Box<dyn std::error::Error>> {
            let proj_contents = fs::read_to_string(&proj_file)?;
            let project: Project = toml::from_str(&proj_contents)?;
            Ok((project.id.clone(), project))
        })
        .collect::<Result<_, _>>()?;

    let tickets: HashMap<_, _> = toml_utils::get_toml_files_in_dir(&ticket_path)?
        .into_iter()
        .map(|ticket_file| -> Result<_, Box<dyn std::error::Error>> {
            let ticket_contents = fs::read_to_string(&ticket_file)?;
            let ticket: Ticket = toml::from_str(&ticket_contents)?;
            Ok((ticket.id.clone(), ticket))
        })
        .collect::<Result<_, _>>()?;

    Ok(InRepoDB::new(projects, tickets))
}
