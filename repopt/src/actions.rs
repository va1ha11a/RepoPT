use crate::in_repo_db;

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

pub(super) fn list_all_tickets() -> Result<()> {
    println!("Listing all tickets");
    let in_repo_db = in_repo_db::collect_in_repo_db();
    Ok(in_repo_db?
        .iter_tickets()
        .for_each(|t| println!("{:#?}", t)))
}

pub(super) fn show_ticket(id: String) -> Result<()> {
    println!("Showing a ticket with id: {id}");
    let in_repo_db = in_repo_db::collect_in_repo_db();
    let in_repo_db = in_repo_db?;
    let ticket = in_repo_db.get_ticket(&id);
    if let Some(ticket) = ticket {
        println!("{:#?}", ticket);
    } else {
        return Err(From::from("Ticket not found."));
    };
    Ok(())
}

pub(super) fn init_new_repository() -> Result<()> {
    println!("Initializing a new repository");
    let in_repo_db = in_repo_db::collect_in_repo_db();
    // in_repo_db should be an error here
    if in_repo_db.is_ok() {
        return Err(From::from("Repository already exists."));
    }
    Ok(())
}
