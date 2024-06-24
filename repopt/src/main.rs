mod project_data {
    use serde::Deserialize;
    use serde_json::Value;
    use std::collections::HashMap;

    #[derive(Deserialize)]
    pub(crate) struct Project {
        pub(crate) id: String,
        pub(crate) name: String,
        pub(crate) description: String,
        // Other fields...
        #[serde(flatten)]
        pub(crate) extra: HashMap<String, Value>,
    }

    #[derive(Deserialize)]
    pub(crate) struct Ticket {
        pub(crate) id: String,
        pub(crate) title: String,
        pub(crate) status: String,
        #[serde(rename = "type")]
        pub(crate) ticket_type: String,
        // Other fields...
        #[serde(flatten)]
        pub(crate) extra: HashMap<String, Value>,
    }
}

use project_data::{Project, Ticket};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the TOML file
    let proj_contents = fs::read_to_string("example_data/projects/P0001.toml")?;
    let ticket_contents = fs::read_to_string("example_data/tickets/T0001.toml")?;

    // Deserialize the TOML string into the Project struct
    let project: Project = toml::from_str(&proj_contents)?;
    let ticket: Ticket = toml::from_str(&ticket_contents)?;

    // Use the deserialized data (example)
    println!("Project ID: {}", project.id);
    println!("Project Name: {}", project.name);
    println!("Project Description: {}", project.description);
    println!("Extra fields: {:?}", project.extra);

    println!("Ticket ID: {}", ticket.id);
    println!("Ticket Title: {}", ticket.title);
    println!("Ticket Status: {}", ticket.status);
    println!("Ticket Type: {}", ticket.ticket_type);
    println!("Extra fields: {:?}", ticket.extra);

    Ok(())
}
