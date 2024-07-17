mod git_utils;
pub mod structs;
mod toml_utils;

use serde::Serialize;
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use structs::{InRepoDB, Project, ProjectId, Ticket, TicketId};

use crate::config::CONFIG;

const PROJECTS_DIR: &str = "projects";
const TICKETS_DIR: &str = "tickets";

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

fn ensure_dir_exists(dir: &Path) -> Result<()> {
    if !dir.exists() {
        fs::create_dir(dir)?;
    }
    Ok(())
}

#[derive(Clone)]
struct IRDBPaths {
    tickets: PathBuf,
    projects: PathBuf,
}

static IRDB_PATHS: OnceLock<IRDBPaths> = OnceLock::new();

fn get_or_create_irdb_dirs() -> Result<IRDBPaths> {
    let existing_paths = IRDB_PATHS.get();
    if let Some(paths) = existing_paths {
        return Ok(paths.clone());
    }
    let config = CONFIG.get().ok_or("Config not initialized")?;
    let mut base_dir = config.irdb_path.clone();
    if base_dir.is_relative() {
        base_dir = git_utils::find_git_root(env::current_dir()?)?.join(base_dir);
    }
    let tickets_dir = base_dir.join(TICKETS_DIR);
    let projects_dir = base_dir.join(PROJECTS_DIR);
    [&base_dir, &tickets_dir, &projects_dir]
        .iter()
        .try_for_each(|dir| ensure_dir_exists(dir))?;
    let irdb_paths = IRDBPaths {
        tickets: tickets_dir,
        projects: projects_dir,
    };
    let _ = IRDB_PATHS.set(irdb_paths.clone());
    Ok(irdb_paths)
}

pub(super) fn collect_in_repo_db() -> Result<InRepoDB> {
    let irdb_paths = get_or_create_irdb_dirs()?;
    let projects = collect_projects(&irdb_paths.projects)?;
    let tickets = collect_tickets(&irdb_paths.tickets)?;

    Ok(InRepoDB::new(projects, tickets))
}

fn collect_tickets(ticket_path: &Path) -> Result<BTreeMap<TicketId, Ticket>> {
    let tickets: BTreeMap<_, _> = toml_utils::get_toml_files_in_dir(ticket_path)?
        .into_iter()
        .map(|ticket_file| -> Result<_> {
            let ticket_contents = fs::read_to_string(ticket_file)?;
            let ticket: Ticket = toml::from_str(&ticket_contents)?;
            Ok((ticket.id().clone(), ticket))
        })
        .collect::<Result<_>>()?;
    Ok(tickets)
}

fn collect_projects(project_path: &Path) -> Result<BTreeMap<ProjectId, Project>> {
    let projects: BTreeMap<ProjectId, Project> = toml_utils::get_toml_files_in_dir(project_path)?
        .into_iter()
        .map(|proj_file| -> Result<_> {
            let proj_contents = fs::read_to_string(proj_file)?;
            let project: Project = toml::from_str(&proj_contents)?;
            Ok((project.id().clone(), project))
        })
        .collect::<Result<_>>()?;
    Ok(projects)
}

pub(crate) trait IRDBWritableObject: Serialize {
    fn fmt_stub(&self) -> String;
    fn select_path(&self) -> Result<PathBuf>;
}

impl IRDBWritableObject for Ticket {
    fn fmt_stub(&self) -> String {
        self.id().to_string()
    }
    fn select_path(&self) -> Result<PathBuf> {
        let irdb_paths = get_or_create_irdb_dirs()?;
        Ok(irdb_paths.tickets)
    }
}

impl IRDBWritableObject for Project {
    fn fmt_stub(&self) -> String {
        self.id().to_string()
    }
    fn select_path(&self) -> Result<PathBuf> {
        let irdb_paths = get_or_create_irdb_dirs()?;
        Ok(irdb_paths.projects)
    }
}

pub(crate) fn verify_and_write<T: IRDBWritableObject>(item: &T) -> Result<()> {
    let toml_string = toml::to_string(item)?;
    let file_name = format!("{}.toml", item.fmt_stub());
    let save_path = item.select_path()?;

    let mut file = File::create(save_path.join(file_name))?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}
