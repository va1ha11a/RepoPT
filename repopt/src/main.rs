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
        BaseCommands::Init => actions::init_new_repository(),
        BaseCommands::Add => unimplemented!(),
        BaseCommands::List => actions::list_all_tickets(None, None),
        BaseCommands::Show { id } => actions::show_ticket(id),
        BaseCommands::Edit => unimplemented!(),
        BaseCommands::Close => unimplemented!(),
        BaseCommands::Reopen => unimplemented!(),
    }?;

    Ok(())
}

mod actions;
