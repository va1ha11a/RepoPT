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

#[derive(Display, Deserialize, Debug)]
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

    pub fn get_tickets_for_project(&self, project_id: &str) -> Vec<&Ticket> {
        self.tickets
            .values()
            .filter(|ticket| ticket.project.id == project_id)
            .collect()
    }

    pub fn get_open_tickets(&self) -> Vec<&Ticket> {
        self.tickets
            .values()
            .filter(|ticket| ticket.status == TicketStatus::Open)
            .collect()
    }
}
