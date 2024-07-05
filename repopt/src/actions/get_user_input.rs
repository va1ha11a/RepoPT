use inquire::{Select, Text};

use super::{get_projects, list_ticket_by_status};

use crate::in_repo_db_structs::{ProjectStub, Ticket, TicketStatus, TicketType};

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

pub(super) fn get_title() -> Result<String> {
    let title = Text::new("Enter Ticket Title:").prompt()?;
    Ok(title)
}

pub(super) fn get_proj_name() -> Result<String> {
    let title = Text::new("Enter Project Name:").prompt()?;
    Ok(title)
}

pub(super) fn get_proj_desc() -> Result<String> {
    let title = Text::new("Enter Project Description:").prompt()?;
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
    Ok(selected_project.into())
}

pub(super) fn select_open_ticket() -> Result<Ticket> {
    let tickets = list_ticket_by_status(TicketStatus::Open)?;
    select_tickets(tickets)
}

pub(super) fn select_closed_ticket() -> Result<Ticket> {
    let tickets = list_ticket_by_status(TicketStatus::Closed)?;
    select_tickets(tickets)
}

fn select_tickets(tickets: Vec<Ticket>) -> Result<Ticket> {
    let options: Vec<String> = tickets.iter().map(|ticket| ticket.title.clone()).collect();
    let ans = Select::new("Select a ticket:", options).prompt()?;
    let selected_ticket = tickets
        .into_iter()
        .find(|ticket| ticket.title == ans)
        .ok_or("Invalid Ticket")?;
    Ok(selected_ticket)
}
