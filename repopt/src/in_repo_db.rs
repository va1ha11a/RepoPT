use crate::in_repo_db_structs::{InRepoDB, Project, Ticket};
use crate::toml_utils;

use std::collections::HashMap;
use std::fs;

const BASE_DIR: &str = "example_data";
const PROJECTS_DIR: &str = "projects";
const TICKETS_DIR: &str = "tickets";

use std::path::PathBuf;

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

pub(super) fn collect_in_repo_db() -> Result<InRepoDB> {
    let project_path = PathBuf::from(BASE_DIR).join(PROJECTS_DIR);
    let ticket_path = PathBuf::from(BASE_DIR).join(TICKETS_DIR);

    let projects: HashMap<String, Project> = toml_utils::get_toml_files_in_dir(&project_path)?
        .into_iter()
        .map(|proj_file| -> Result<_> {
            let proj_contents = fs::read_to_string(proj_file)?;
            let project: Project = toml::from_str(&proj_contents)?;
            Ok((project.id.clone(), project))
        })
        .collect::<Result<_>>()?;

    let tickets: HashMap<_, _> = toml_utils::get_toml_files_in_dir(&ticket_path)?
        .into_iter()
        .map(|ticket_file| -> Result<_> {
            let ticket_contents = fs::read_to_string(ticket_file)?;
            let ticket: Ticket = toml::from_str(&ticket_contents)?;
            Ok((ticket.id.clone(), ticket))
        })
        .collect::<Result<_>>()?;

    Ok(InRepoDB::new(projects, tickets))
}
