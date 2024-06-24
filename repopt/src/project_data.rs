use derive_more::Display;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Project {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: String,
    // Other fields...
    #[serde(flatten)]
    pub(crate) extra: HashMap<String, Value>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct ProjectStub {
    pub(crate) id: String,
}

#[derive(Display, Deserialize, Debug)]
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
    pub(crate) title: String,
    pub(crate) status: TicketStatus,
    #[serde(rename = "type")]
    pub(crate) ticket_type: TicketType,
    pub(crate) project: ProjectStub,
    // Other fields...
    #[serde(flatten)]
    pub(crate) extra: HashMap<String, Value>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct InRepoDB {
    pub(crate) projects: HashMap<String, Project>,
    pub(crate) tickets: HashMap<String, Ticket>,
}

impl InRepoDB {
    pub fn new(projects: HashMap<String, Project>, tickets: HashMap<String, Ticket>) -> Self {
        InRepoDB { projects, tickets }
    }
}
