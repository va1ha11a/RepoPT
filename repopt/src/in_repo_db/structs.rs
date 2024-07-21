pub(crate) use project::{Project, ProjectDescription, ProjectId, ProjectName};
use serde::Deserialize;
use std::collections::BTreeMap;
pub(crate) mod project;
pub(crate) use ticket::*;
pub(crate) mod ticket;

// ###### InRepoDB Section ######
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
        Some(format!("T{next_id:04}").into())
    }

    pub fn get_next_project_id(&self) -> Option<ProjectId> {
        let last_id = self.projects.last_key_value().map(|(id, _)| id)?;
        let next_id = last_id.to_string().get(1..)?.parse::<u16>().unwrap() + 1;
        Some(ProjectId(format!("P{next_id:04}")))
    }
}

#[cfg(test)]
mod tests;
