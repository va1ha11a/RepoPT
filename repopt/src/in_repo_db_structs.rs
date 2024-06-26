use derive_more::Display;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Project {
    pub(crate) id: String,
    name: String,
    description: String,
    // Other fields...
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct ProjectStub {
    pub(crate) id: String,
}

#[derive(Display, Deserialize, Debug, PartialEq, Eq)]
pub(crate) enum TicketStatus {
    #[display(fmt = "Open")]
    Open,
    #[display(fmt = "In Progress")]
    InProgress,
    #[display(fmt = "Closed")]
    Closed,
}

#[derive(Display, Deserialize, Debug, PartialEq, Eq)]
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Ticket {
    pub(crate) id: String,
    title: String,
    status: TicketStatus,
    #[serde(rename = "type")]
    ticket_type: TicketType,
    project: ProjectStub,
    // Other fields...
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[allow(dead_code)]
impl Ticket {
    pub(crate) fn is_open(&self) -> bool {
        self.status == TicketStatus::Open
    }

    pub(crate) fn get_project_id(&self) -> &str {
        &self.project.id
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct InRepoDB {
    projects: HashMap<String, Project>,
    tickets: HashMap<String, Ticket>,
}

#[allow(dead_code)]
impl InRepoDB {
    pub fn new(projects: HashMap<String, Project>, tickets: HashMap<String, Ticket>) -> Self {
        InRepoDB { projects, tickets }
    }

    pub fn get_project(&self, id: &str) -> Option<&Project> {
        self.projects.get(id)
    }

    pub fn get_ticket(&self, id: &str) -> Option<&Ticket> {
        self.tickets.get(id)
    }

    pub fn iter_tickets(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.values()
    }
}

pub(super) trait TicketFilters<'a>: Iterator<Item = &'a Ticket> + Sized
where
    Self: 'a,
{
    fn with_status(self, status: &'a TicketStatus) -> Box<dyn Iterator<Item = &'a Ticket> + 'a> {
        Box::new(self.filter(move |ticket| ticket.status == *status))
    }

    fn with_type(self, ticket_type: &'a TicketType) -> Box<dyn Iterator<Item = &'a Ticket> + 'a> {
        Box::new(self.filter(move |ticket| ticket.ticket_type == *ticket_type))
    }
}

// Implement the trait for all iterators that return a ticket reference with the same lifetime
impl<'a, T> TicketFilters<'a> for T where T: Iterator<Item = &'a Ticket> + 'a {}
