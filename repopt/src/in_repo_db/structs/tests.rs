use super::*;

fn setup_in_repo_db_one() -> InRepoDB {
    let project_id = ProjectId("P0001".to_string());
    let project = Project {
        id: project_id.clone(),
        name: "Test Project".to_string().into(),
        description: "Test Description".to_string().into(),
        extra: HashMap::new(),
    };

    let ticket_id = TicketId("T0001".to_string());
    let ticket = Ticket {
        id: ticket_id.clone(),
        title: "Test Ticket".to_string().into(),
        status: TicketStatus::Backlog,
        ticket_type: TicketType::Bug,
        project: ProjectStub {
            id: project_id.clone(),
        },
        extra: HashMap::new(),
    };

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
    assert_eq!(project.is_some(), true);
}

#[test]
fn test_get_ticket() {
    let in_repo_db = setup_in_repo_db_one();
    let ticket_id = TicketId("T0001".to_string());
    let ticket = in_repo_db.get_ticket(&ticket_id);
    assert_eq!(ticket.is_some(), true);
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
