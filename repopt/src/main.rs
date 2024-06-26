use clap::Parser;

mod in_repo_db;
mod in_repo_db_structs;
mod toml_utils;

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug)]
enum BaseCommands {
    #[clap(name = "init", about = "Initialize a new repository")]
    Init,
    #[clap(name = "add", about = "Add a new ticket")]
    Add,
    #[clap(name = "list", about = "List all tickets")]
    List,
    #[clap(name = "show", about = "Show a ticket")]
    Show { id: String },
    #[clap(name = "edit", about = "Edit a ticket")]
    Edit,
    #[clap(name = "close", about = "Close a ticket")]
    Close,
    #[clap(name = "reopen", about = "Reopen a ticket")]
    Reopen,
}

fn main() -> Result<()> {
    let base_command = BaseCommands::parse();

    match base_command {
        BaseCommands::Init => init_new_repository(),
        BaseCommands::Add => add_ticket(),
        BaseCommands::List => list_all_tickets(),
        BaseCommands::Show { id } => show_ticket(id),
        BaseCommands::Edit => unimplemented!(),
        BaseCommands::Close => unimplemented!(),
        BaseCommands::Reopen => unimplemented!(),
    }?;

    // Deserialize the data
    //let in_repo_db = in_repo_db::collect_in_repo_db()?;

    // Use the deserialized data (example)

    // println!("Project one Open Tickets:");
    // let tiks: Vec<_> = in_repo_db
    //     .iter_tickets()
    //     .filter(|t| t.get_project_id() == "P0001")
    //     .filter(|t| t.is_open())
    //     .collect();
    // println!("{:#?}", tiks);

    // println!("Project two Closed Tickets:");
    // let tiks: Vec<_> = in_repo_db
    //     .iter_tickets()
    //     .filter(|t| t.get_project_id() == "P0002")
    //     .filter(|t| !t.is_open())
    //     .collect();
    // println!("{:#?}", tiks);

    Ok(())
}

fn list_all_tickets() -> Result<()> {
    println!("Listing all tickets");
    let in_repo_db = in_repo_db::collect_in_repo_db();
    Ok(in_repo_db?
        .iter_tickets()
        .for_each(|t| println!("{:#?}", t)))
}

fn show_ticket(id: String) -> Result<()> {
    println!("Showing a ticket with id: {id}");
    let in_repo_db = in_repo_db::collect_in_repo_db();
    let in_repo_db = in_repo_db?;
    let ticket = in_repo_db.get_ticket(&id);
    Ok(if let Some(ticket) = ticket {
        println!("{:#?}", ticket);
    } else {
        return Err(From::from("Ticket not found."));
    })
}

fn init_new_repository() -> Result<()> {
    println!("Initializing a new repository");
    let in_repo_db = in_repo_db::collect_in_repo_db();
    // in_repo_db should be an error here
    if in_repo_db.is_ok() {
        return Err(From::from("Repository already exists."));
    }
    Ok(())
}

fn add_ticket() -> Result<()> {
    // let in_repo_db = in_repo_db::collect_in_repo_db();
    unimplemented!()
}
