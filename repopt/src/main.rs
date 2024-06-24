mod project_data {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub(crate) struct Project {
        pub(crate) id: String,
        pub(crate) name: String,
        pub(crate) description: String,
    }
}

use project_data::Project;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the TOML file
    let contents = fs::read_to_string("example_data/projects/P0001.toml")?;

    // Deserialize the TOML string into the Project struct
    let project: Project = toml::from_str(&contents)?;

    // Use the deserialized data (example)
    println!("Project ID: {}", project.id);
    println!("Project Name: {}", project.name);
    println!("Project Description: {}", project.description);

    Ok(())
}
