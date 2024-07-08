use clap::ValueEnum;
use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use typed_builder::TypedBuilder;

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
#[from(forward)]
pub(crate) struct ProjectId(String);
#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
#[from(forward)]
pub(crate) struct ProjectName(String);
#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
#[from(forward)]
pub(crate) struct ProjectDescription(String);

#[derive(TypedBuilder)]
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, Display)]
#[display(
    fmt = "Project ID: {}\nName: {}\nDescription: {}",
    id,
    name,
    description
)]
pub(crate) struct Project {
    id: ProjectId,
    name: ProjectName,
    description: ProjectDescription,
    // Other fields...
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl Project {
    pub(crate) fn id(&self) -> &ProjectId {
        &self.id
    }

    pub(crate) fn name(&self) -> &ProjectName {
        &self.name
    }
}

impl From<Project> for ProjectStub {
    fn from(project: Project) -> Self {
        ProjectStub { id: project.id }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Display, Clone)]
pub(crate) struct ProjectStub {
    id: ProjectId,
}

#[derive(Display, Serialize, Deserialize, Debug, PartialEq, Eq, ValueEnum, Clone)]
pub(crate) enum TicketStatus {
    #[display(fmt = "Backlog")]
    Backlog,
    #[display(fmt = "In Progress")]
    InProgress,
    #[display(fmt = "Closed")]
    Closed,
}

impl From<TicketStatus> for Vec<TicketStatus> {
    fn from(status: TicketStatus) -> Vec<TicketStatus> {
        vec![status]
    }
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
#[from(forward)]
pub(crate) struct TicketId(String);

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
#[from(forward)]
pub(crate) struct TicketTitle(String);

#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone, Display)]
#[display(
    fmt = "ID: {}\nTitle: {}\nStatus: {}\nType: {}\nProject ID: {}",
    id,
    title,
    status,
    ticket_type,
    project
)]
pub(crate) struct Ticket {
    id: TicketId,
    title: TicketTitle,
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
    pub(crate) fn id(&self) -> &TicketId {
        &self.id
    }

    pub(crate) fn title(&self) -> &TicketTitle {
        &self.title
    }

    pub(crate) fn is_open(&self) -> bool {
        self.status != TicketStatus::Closed
    }

    pub(crate) fn get_project_id(&self) -> &ProjectId {
        &self.project.id
    }

    pub(crate) fn close(&mut self) {
        self.status = TicketStatus::Closed;
    }

    pub(crate) fn reopen(&mut self, status: Option<TicketStatus>) {
        self.status = status.unwrap_or(TicketStatus::InProgress);
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
    fn with_status<S>(self, status: S) -> Box<dyn Iterator<Item = &'a Ticket> + 'a>
    where
        S: Into<Vec<TicketStatus>>,
    {
        let statuses = status.into();
        Box::new(self.filter(move |ticket| statuses.contains(&ticket.status)))
    }

    fn with_type(self, ticket_type: TicketType) -> Box<dyn Iterator<Item = &'a Ticket> + 'a> {
        Box::new(self.filter(move |ticket| ticket.ticket_type == ticket_type))
    }
}

// Implement the trait for all iterators that return a ticket reference with the same lifetime
impl<'a, T> TicketFilters<'a> for T where T: Iterator<Item = &'a Ticket> + 'a {}

#[cfg(test)]
mod tests;
