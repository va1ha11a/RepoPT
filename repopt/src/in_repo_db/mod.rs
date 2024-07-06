pub mod structs;
mod toml_utils;

use structs::{InRepoDB, Project, ProjectId, Ticket, TicketId};

use serde::Serialize;

use std::collections::HashMap;
use std::fs::{self, File};

const BASE_DIR: &str = "example_data";
const PROJECTS_DIR: &str = "projects";
const TICKETS_DIR: &str = "tickets";

use std::io::Write;
use std::path::{Path, PathBuf};

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

pub(super) fn collect_in_repo_db() -> Result<InRepoDB> {
    let project_path = PathBuf::from(BASE_DIR).join(PROJECTS_DIR);
    let ticket_path = PathBuf::from(BASE_DIR).join(TICKETS_DIR);

    let projects = collect_projects(&project_path)?;

    let tickets = collect_tickets(&ticket_path)?;

    Ok(InRepoDB::new(projects, tickets))
}

fn collect_tickets(ticket_path: &Path) -> Result<HashMap<TicketId, Ticket>> {
    let tickets: HashMap<_, _> = toml_utils::get_toml_files_in_dir(ticket_path)?
        .into_iter()
        .map(|ticket_file| -> Result<_> {
            let ticket_contents = fs::read_to_string(ticket_file)?;
            let ticket: Ticket = toml::from_str(&ticket_contents)?;
            Ok((ticket.id.clone(), ticket))
        })
        .collect::<Result<_>>()?;
    Ok(tickets)
}

fn collect_projects(project_path: &Path) -> Result<HashMap<ProjectId, Project>> {
    let projects: HashMap<ProjectId, Project> = toml_utils::get_toml_files_in_dir(project_path)?
        .into_iter()
        .map(|proj_file| -> Result<_> {
            let proj_contents = fs::read_to_string(proj_file)?;
            let project: Project = toml::from_str(&proj_contents)?;
            Ok((project.id.clone(), project))
        })
        .collect::<Result<_>>()?;
    Ok(projects)
}

pub(crate) trait IRDBWritableObject: Serialize {
    fn fmt_stub(&self) -> String;
    fn select_path(&self) -> PathBuf;
}

impl IRDBWritableObject for Ticket {
    fn fmt_stub(&self) -> String {
        self.id.to_string()
    }
    fn select_path(&self) -> PathBuf {
        PathBuf::from(BASE_DIR).join(TICKETS_DIR)
    }
}

impl IRDBWritableObject for Project {
    fn fmt_stub(&self) -> String {
        self.id.to_string()
    }
    fn select_path(&self) -> PathBuf {
        PathBuf::from(BASE_DIR).join(PROJECTS_DIR)
    }
}

pub(crate) fn verify_and_write<T: IRDBWritableObject>(item: &T) -> Result<()> {
    let toml_string = toml::to_string(item)?;
    let file_name = format!("{}.toml", item.fmt_stub());
    let save_path = item.select_path();

    let mut file = File::create(save_path.join(file_name))?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}
