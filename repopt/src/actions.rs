use std::collections::HashMap;
use ulid::Ulid;

use crate::in_repo_db;
use crate::in_repo_db_structs::{Ticket, TicketFilters, TicketStatus, TicketType};

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

pub(super) fn list_all_tickets(
    filter_on_status: Option<TicketStatus>,
    filter_on_type: Option<TicketType>,
) -> Result<()> {
    println!("Listing all tickets");
    let in_repo_db = in_repo_db::collect_in_repo_db();
    let binding = in_repo_db?;
    let mut iter = Box::new(binding.iter_tickets()) as Box<dyn Iterator<Item = &Ticket>>;
    if let Some(status) = filter_on_status {
        iter = iter.with_status(status);
    }
    if let Some(ticket_type) = filter_on_type {
        iter = iter.with_type(ticket_type);
    }
    Ok(iter.for_each(|t| println!("{:#?}", t)))
}

pub(super) fn show_ticket(id: String) -> Result<()> {
    println!("Showing a ticket with id: {id}");
    let in_repo_db = in_repo_db::collect_in_repo_db();
    let in_repo_db = in_repo_db?;
    let ticket = in_repo_db.get_ticket(id.try_into()?);
    if let Some(ticket) = ticket {
        println!("{:#?}", ticket);
    } else {
        return Err(From::from("Ticket not found."));
    };
    Ok(())
}

pub(super) fn init_new_repository() -> Result<()> {
    println!("Initializing a new repository");
    let in_repo_db = in_repo_db::collect_in_repo_db();
    // in_repo_db should be an error here
    if in_repo_db.is_ok() {
        return Err(From::from("Repository already exists."));
    }
    Ok(())
}

pub(super) fn add_new_ticket(
    title: Option<String>,
    status: Option<TicketStatus>,
    ticket_type: Option<TicketType>,
) -> Result<()> {
    let ticket = Ticket::builder()
        .id(Ulid::new().to_string().try_into()?)
        .project(get_user_input::get_project_id().unwrap())
        .title(title.unwrap_or_else(|| get_user_input::get_title().unwrap()))
        .status(status.unwrap_or_else(|| get_user_input::get_ticket_status().unwrap()))
        .ticket_type(ticket_type.unwrap_or_else(|| get_user_input::get_ticket_type().unwrap()))
        .extra(HashMap::new())
        .build();
    println!("{}", ticket);
    in_repo_db::verify_and_write(ticket)?;

    Ok(())
}

mod get_user_input {
    use inquire::{Select, Text};

    use crate::{
        in_repo_db,
        in_repo_db_structs::{Project, ProjectStub, TicketStatus, TicketType},
    };

    type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
    type Result<T> = std::result::Result<T, Error>;

    pub(super) fn get_title() -> Result<String> {
        let title = Text::new("Enter Ticket Title:").prompt()?;
        Ok(title)
    }

    pub(super) fn get_ticket_type() -> Result<TicketType> {
        let options = vec!["Bug", "Feature"];
        let ans = Select::new("Select Ticket Type:", options).prompt();
        match ans {
            Ok("Bug") => Ok(TicketType::Bug),
            Ok("Feature") => Ok(TicketType::Feature),
            _ => Err(From::from("Invalid Ticket Type")),
        }
    }

    pub(super) fn get_ticket_status() -> Result<TicketStatus> {
        let options = vec!["Open", "In Progress", "Closed"];
        let ans = Select::new("Select Ticket Status:", options).prompt();
        match ans {
            Ok("Open") => Ok(TicketStatus::Open),
            Ok("In Progress") => Ok(TicketStatus::InProgress),
            Ok("Closed") => Ok(TicketStatus::Closed),
            _ => Err(From::from("Invalid Ticket Status")),
        }
    }

    pub(super) fn get_project_id() -> Result<ProjectStub> {
        let projects = get_projects()?;
        let options: Vec<String> = projects
            .iter()
            .map(|project| project.name.clone())
            .collect();
        let ans = Select::new("Select a project:", options).prompt()?;
        let selected_project = projects
            .into_iter()
            .find(|project| project.name == ans)
            .ok_or("Invalid Project")?;
        Ok(project_to_projectstub(selected_project))
    }

    fn get_projects() -> Result<Vec<Project>> {
        let in_repo_db = in_repo_db::collect_in_repo_db();
        let in_repo_db = in_repo_db?;
        let projects = in_repo_db.iter_projects().cloned().collect();
        Ok(projects)
    }

    fn project_to_projectstub(project: Project) -> ProjectStub {
        ProjectStub { id: project.id }
    }
}
