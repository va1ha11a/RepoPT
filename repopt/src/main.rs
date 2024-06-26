mod in_repo_db;
mod in_repo_db_structs;
mod toml_utils;

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let in_repo_db = in_repo_db::collect_in_repo_db()?;

    // Use the deserialized data (example)

    println!("Project one Open Tickets:");
    let tiks: Vec<_> = in_repo_db
        .iter_tickets()
        .filter(|t| t.get_project_id() == "P0001")
        .filter(|t| t.is_open())
        .collect();
    println!("{:#?}", tiks);

    println!("Project two Closed Tickets:");
    let tiks: Vec<_> = in_repo_db
        .iter_tickets()
        .filter(|t| t.get_project_id() == "P0002")
        .filter(|t| !t.is_open())
        .collect();
    println!("{:#?}", tiks);

    Ok(())
}
