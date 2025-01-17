use super::{Project, ProjectId};
use clap::ValueEnum;
use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

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
pub(crate) struct TicketId(pub String);

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
#[from(forward)]
pub(crate) struct TicketTitle(String);

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash, Display, From, PartialOrd, Ord,
)]
#[from(forward)]
pub(crate) struct TicketDescription(String);

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Display, Clone, PartialEq, Eq)]
pub(crate) struct ProjectStub {
    id: ProjectId,
}

impl From<&Project> for ProjectStub {
    fn from(project: &Project) -> Self {
        ProjectStub {
            id: project.id().to_owned(),
        }
    }
}

#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone, Display)]
#[display(
    fmt = "ID: {id}\nTitle: {title}\nStatus: {status}\nType: {ticket_type}\nProject ID: {project}"
)]
#[allow(clippy::struct_field_names)]
pub(crate) struct Ticket {
    id: TicketId,
    title: TicketTitle,
    description: TicketDescription,
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

    fn for_project(self, project: ProjectId) -> Box<dyn Iterator<Item = &'a Ticket> + 'a> {
        Box::new(self.filter(move |ticket| ticket.project.id == project))
    }
}

// Implement the trait for all iterators that return a ticket reference with the same lifetime
impl<'a, T> TicketFilters<'a> for T where T: Iterator<Item = &'a Ticket> + 'a {}
