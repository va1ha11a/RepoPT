use clap::ValueEnum;
use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Project {
    pub(crate) id: ProjectId,
    pub(crate) name: String, // TODO: Create NewType
    description: String,     // TODO: Create NewType
    // Other fields...
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl From<Project> for ProjectStub {
    fn from(project: Project) -> Self {
        ProjectStub { id: project.id }
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Project ID: {}\nName: {}\nDescription: {}",
            self.id, self.name, self.description
        )
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Display, Clone)]
pub(crate) struct ProjectStub {
    pub(crate) id: ProjectId,
}

#[derive(Display, Serialize, Deserialize, Debug, PartialEq, Eq, ValueEnum, Clone)]
pub(crate) enum TicketStatus {
    #[display(fmt = "Open")]
    Open,
    #[display(fmt = "In Progress")]
    InProgress,
    #[display(fmt = "Closed")]
    Closed,
}

#[derive(Display, Serialize, Deserialize, Debug, PartialEq, Eq, ValueEnum, Clone)]
pub(crate) enum TicketType {
    #[display(fmt = "Bug")]
    Bug,
    #[display(fmt = "Feature")]
    Feature,
    #[display(fmt = "Documentation")]
    Documentation,
    #[display(fmt = "Other")]
    Other,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
pub(crate) struct TicketId(String);

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
pub(crate) struct ProjectId(String);

#[derive(TypedBuilder)]
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Ticket {
    pub(crate) id: TicketId,
    pub(crate) title: String, // TODO: Create NewType
    status: TicketStatus,
    #[serde(rename = "type")]
    ticket_type: TicketType,
    project: ProjectStub,
    // Other fields...
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ticket ID: {}\nTitle: {}\nStatus: {}\nType: {}\nProject ID: {}",
            self.id, self.title, self.status, self.ticket_type, self.project.id
        )
    }
}

#[allow(dead_code)]
impl Ticket {
    pub(crate) fn is_open(&self) -> bool {
        self.status == TicketStatus::Open
    }

    pub(crate) fn get_project_id(&self) -> &ProjectId {
        &self.project.id
    }

    pub(crate) fn close(&mut self) {
        self.status = TicketStatus::Closed;
    }

    pub(crate) fn reopen(&mut self) {
        self.status = TicketStatus::Open;
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct InRepoDB {
    projects: BTreeMap<ProjectId, Project>,
    tickets: BTreeMap<TicketId, Ticket>,
}

#[allow(dead_code)]
impl InRepoDB {
    pub fn new(
        projects: BTreeMap<ProjectId, Project>,
        tickets: BTreeMap<TicketId, Ticket>,
    ) -> Self {
        InRepoDB { projects, tickets }
    }

    pub fn get_project(&self, id: &ProjectId) -> Option<&Project> {
        self.projects.get(id)
    }

    pub fn iter_projects(&self) -> impl Iterator<Item = &Project> {
        self.projects.values()
    }

    pub fn get_ticket(&self, id: &TicketId) -> Option<&Ticket> {
        self.tickets.get(id)
    }

    pub fn iter_tickets(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.values()
    }

    pub fn get_next_ticket_id(&self) -> Option<TicketId> {
        let last_id = self.tickets.last_key_value().map(|(id, _)| id)?;
        let next_id = last_id.to_string().get(1..)?.parse::<u16>().unwrap() + 1;
        Some(TicketId(format!("T{:04}", next_id)))
    }

    pub fn get_next_project_id(&self) -> Option<ProjectId> {
        let last_id = self.projects.last_key_value().map(|(id, _)| id)?;
        let next_id = last_id.to_string().get(1..)?.parse::<u16>().unwrap() + 1;
        Some(ProjectId(format!("P{:04}", next_id)))
    }
}

pub(crate) trait TicketFilters<'a>: Iterator<Item = &'a Ticket> + Sized
where
    Self: 'a,
{
    fn with_status(self, status: TicketStatus) -> Box<dyn Iterator<Item = &'a Ticket> + 'a> {
        Box::new(self.filter(move |ticket| ticket.status == status))
    }

    fn with_type(self, ticket_type: TicketType) -> Box<dyn Iterator<Item = &'a Ticket> + 'a> {
        Box::new(self.filter(move |ticket| ticket.ticket_type == ticket_type))
    }
}

// Implement the trait for all iterators that return a ticket reference with the same lifetime
impl<'a, T> TicketFilters<'a> for T where T: Iterator<Item = &'a Ticket> + 'a {}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_in_repo_db_one() -> InRepoDB {
        let project_id = ProjectId("P0001".to_string());
        let project = Project {
            id: project_id.clone(),
            name: "Test Project".to_string(),
            description: "Test Description".to_string(),
            extra: HashMap::new(),
        };

        let ticket_id = TicketId("T0001".to_string());
        let ticket = Ticket {
            id: ticket_id.clone(),
            title: "Test Ticket".to_string(),
            status: TicketStatus::Open,
            ticket_type: TicketType::Bug,
            project: ProjectStub {
                id: project_id.clone(),
            },
            extra: HashMap::new(),
        };

        InRepoDB {
            projects: vec![(project_id, project)].into_iter().collect(),
            tickets: vec![(ticket_id, ticket)].into_iter().collect(),
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
        let open_tickets: Vec<&Ticket> = in_repo_db
            .iter_tickets()
            .with_status(TicketStatus::Open)
            .collect();
        assert_eq!(open_tickets.len(), 1);
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
}
