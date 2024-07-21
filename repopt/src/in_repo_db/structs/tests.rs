use std::collections::HashMap;

use super::*;

fn setup_in_repo_db_one() -> InRepoDB {
    let project_id = ProjectId("P0001".to_string());
    let project = Project::builder()
        .id(project_id.clone())
        .name("Test Project".into())
        .description("Test Description".into())
        .extra(HashMap::new())
        .build();

    let ticket_id = TicketId("T0001".to_string());
    let ticket = Ticket::builder()
        .id(ticket_id.clone())
        .title("Test Ticket".into())
        .status(TicketStatus::Backlog)
        .ticket_type(TicketType::Bug)
        .project(ProjectStub::from(&project))
        .extra(HashMap::new())
        .build();

    InRepoDB {
        projects: BTreeMap::from([(project_id, project)]),
        tickets: BTreeMap::from([(ticket_id, ticket)]),
    }
}

#[test]
fn test_get_next_ticket_id() {
    let in_repo_db = setup_in_repo_db_one();
    let next_ticket_id = in_repo_db.get_next_ticket_id();
    assert_eq!(next_ticket_id, Some(TicketId("T0002".to_string())));
}

#[test]
fn test_get_next_project_id() {
    let in_repo_db = setup_in_repo_db_one();
    let next_project_id = in_repo_db.get_next_project_id();
    assert_eq!(next_project_id, Some(ProjectId("P0002".to_string())));
}

#[test]
fn test_get_project() {
    let in_repo_db = setup_in_repo_db_one();
    let project_id = ProjectId("P0001".to_string());
    let project = in_repo_db.get_project(&project_id);
    assert!(project.is_some());
}

#[test]
fn test_get_ticket() {
    let in_repo_db = setup_in_repo_db_one();
    let ticket_id = TicketId("T0001".to_string());
    let ticket = in_repo_db.get_ticket(&ticket_id);
    assert!(ticket.is_some());
}

#[test]
fn test_iter_projects() {
    let in_repo_db = setup_in_repo_db_one();
    let projects: Vec<&Project> = in_repo_db.iter_projects().collect();
    assert_eq!(projects.len(), 1);
}

#[test]
fn test_iter_tickets() {
    let in_repo_db = setup_in_repo_db_one();
    let tickets: Vec<&Ticket> = in_repo_db.iter_tickets().collect();
    assert_eq!(tickets.len(), 1);
}

#[test]
fn test_with_status() {
    let in_repo_db = setup_in_repo_db_one();
    let backlog_tickets: Vec<&Ticket> = in_repo_db
        .iter_tickets()
        .with_status(TicketStatus::Backlog)
        .collect();
    assert_eq!(backlog_tickets.len(), 1);
}

#[test]
fn test_with_type() {
    let in_repo_db = setup_in_repo_db_one();
    let bug_tickets: Vec<&Ticket> = in_repo_db
        .iter_tickets()
        .with_type(TicketType::Bug)
        .collect();
    assert_eq!(bug_tickets.len(), 1);
}
