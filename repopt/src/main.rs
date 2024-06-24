mod project_data;

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
    println!("{:#?}", project);

    println!("{:#?}", ticket);
    Ok(())
}
