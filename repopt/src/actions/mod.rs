mod get_user_input;

use get_user_input::TicketStatusTypes;

use crate::config::CONFIG;
use crate::in_repo_db;
use crate::in_repo_db::structs::{
    Project, ProjectId, Ticket, TicketFilters, TicketStatus, TicketType,
};
use crate::output_formatter::GenerateOutputFormat;
use std::collections::HashMap;

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

pub(super) fn list_all_tickets(
    filter_on_status: Option<TicketStatus>,
    filter_on_type: Option<TicketType>,
    for_project: Option<ProjectId>,
) -> Result<()> {
    let config = CONFIG.get().ok_or("Config not initialized")?;
    let in_repo_db = in_repo_db::collect_in_repo_db();
    let binding = in_repo_db?;
    let mut iter = Box::new(binding.iter_tickets()) as Box<dyn Iterator<Item = &Ticket>>;
    if let Some(status) = filter_on_status {
        iter = iter.with_status(status);
    }
    if let Some(ticket_type) = filter_on_type {
        iter = iter.with_type(ticket_type);
    }
    if let Some(project) = for_project {
        iter = iter.for_project(project);
    }
    let out_string = config
        .formatter
        .try_format_multiple(&iter.collect::<Vec<&Ticket>>())?;
    println!("{out_string}");
    Ok(())
}

pub(super) fn show_ticket(id: String) -> Result<()> {
    let config = CONFIG.get().ok_or("Config not initialized")?;
    let in_repo_db = in_repo_db::collect_in_repo_db();
    let in_repo_db = in_repo_db?;
    let ticket = in_repo_db.get_ticket(&id.into());
    if let Some(ticket) = ticket {
        let out_string = config.formatter.try_format_single(ticket)?;
        println!("{out_string}");
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
    description: Option<String>,
    status: Option<TicketStatus>,
    ticket_type: Option<TicketType>,
) -> Result<()> {
    let in_repo_db = in_repo_db::collect_in_repo_db()?;
    let project_id = get_user_input::get_project_id()?;

    let title = title.map_or_else(get_user_input::get_title, |t| Ok(t.into()))?;
    let description = description.map_or_else(get_user_input::get_description, |t| Ok(t.into()))?;
    let status = status.map_or_else(
        || get_user_input::get_ticket_status(&TicketStatusTypes::All),
        Ok,
    )?;
    let ticket_type = ticket_type.map_or_else(get_user_input::get_ticket_type, Ok)?;

    let ticket = Ticket::builder()
        .id(in_repo_db
            .get_next_ticket_id()
            .unwrap_or("T0001".to_owned().into()))
        .project(project_id)
        .title(title)
        .description(description)
        .status(status)
        .ticket_type(ticket_type)
        .extra(HashMap::new())
        .build();

    println!("{ticket}");
    in_repo_db::verify_and_write(&ticket)?;

    Ok(())
}

pub(super) fn add_new_project(name: Option<String>, description: Option<String>) -> Result<()> {
    println!("Adding a new project");
    let in_repo_db = in_repo_db::collect_in_repo_db()?;
    let name = name.map_or_else(get_user_input::get_proj_name, |t| Ok(t.into()))?;
    let description = description.map_or_else(get_user_input::get_proj_desc, |t| Ok(t.into()))?;
    let project = Project::builder()
        .id(in_repo_db
            .get_next_project_id()
            .unwrap_or("P0001".to_owned().into()))
        .name(name)
        .description(description)
        .extra(HashMap::new())
        .build();
    println!("{project}");
    in_repo_db::verify_and_write(&project)?;
    Ok(())
}

pub(super) fn close_ticket() -> Result<()> {
    println!("Closing a ticket");
    let mut ticket = get_user_input::select_open_ticket()?;
    ticket.close();

    in_repo_db::verify_and_write(&ticket)?;
    Ok(())
}

pub(super) fn reopen_ticket() -> Result<()> {
    println!("Reopening a ticket");
    let mut ticket = get_user_input::select_closed_ticket()?;
    let status = get_user_input::get_ticket_status(&TicketStatusTypes::OnlyOpen)?;
    ticket.reopen(Some(status));
    in_repo_db::verify_and_write(&ticket)?;
    Ok(())
}

pub(super) fn list_ticket_by_status<S>(list_status: S) -> Result<Vec<Ticket>>
where
    S: Into<Vec<TicketStatus>>,
{
    let statuses = list_status.into();
    let in_repo_db = in_repo_db::collect_in_repo_db();
    let in_repo_db = in_repo_db?;
    let tickets = in_repo_db
        .iter_tickets()
        .with_status(statuses)
        .cloned()
        .collect();
    Ok(tickets)
}

pub(super) fn get_projects() -> Result<Vec<Project>> {
    let in_repo_db = in_repo_db::collect_in_repo_db();
    let in_repo_db = in_repo_db?;
    let projects = in_repo_db.iter_projects().cloned().collect();
    Ok(projects)
}
