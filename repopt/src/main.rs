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
    Show,
    #[clap(name = "edit", about = "Edit a ticket")]
    Edit,
    #[clap(name = "close", about = "Close a ticket")]
    Close,
    #[clap(name = "reopen", about = "Reopen a ticket")]
    Reopen,
}

fn main() -> Result<()> {
    let base_command = BaseCommands::parse();

    let in_repo_db = in_repo_db::collect_in_repo_db();

    match base_command {
        BaseCommands::Init => {
            println!("Initializing a new repository");
            // in_repo_db should be an error here
            if in_repo_db.is_ok() {
                return Err(From::from("Repository already exists."));
            }
        }
        BaseCommands::Add => {
            println!("Adding a new ticket");
        }
        BaseCommands::List => {
            println!("Listing all tickets");
            println!("{:#?}", in_repo_db?.iter_tickets().collect::<Vec<_>>());
        }
        BaseCommands::Show => {
            println!("Showing a ticket");
        }
        BaseCommands::Edit => {
            println!("Editing a ticket");
        }
        BaseCommands::Close => {
            println!("Closing a ticket");
        }
        BaseCommands::Reopen => {
            println!("Reopening a ticket");
        }
    }

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
