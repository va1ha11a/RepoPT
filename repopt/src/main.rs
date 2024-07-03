use clap::Parser;

mod in_repo_db;
mod in_repo_db_structs;
mod toml_utils;

use in_repo_db_structs::{TicketStatus, TicketType};

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug)]
enum BaseCommands {
    #[clap(name = "init", about = "Initialize a new repository")]
    Init,
    #[clap(name = "add", about = "Add a new ticket")]
    Add(AddOptions),
    #[clap(name = "list", about = "List all tickets")]
    List(ListOptions),
    #[clap(name = "show", about = "Show a ticket")]
    Show { id: String },
    #[clap(name = "edit", about = "Edit a ticket")]
    Edit,
    #[clap(name = "close", about = "Close a ticket")]
    Close,
    #[clap(name = "reopen", about = "Reopen a ticket")]
    Reopen,
}

#[derive(Parser, Debug)]
struct AddOptions {
    #[clap(long, value_enum)]
    title: Option<String>,
    #[clap(long, value_enum)]
    status: Option<TicketStatus>,
    #[clap(long, value_enum)]
    ticket_type: Option<TicketType>,
}

#[derive(Parser, Debug)]
struct ListOptions {
    #[clap(long, value_enum)]
    status: Option<TicketStatus>,
    #[clap(long, value_enum)]
    ticket_type: Option<TicketType>,
}

fn main() -> Result<()> {
    let base_command = BaseCommands::parse();

    match base_command {
        BaseCommands::Init => actions::init_new_repository(),
        BaseCommands::Add(options) => {
            actions::add_new_ticket(options.title, options.status, options.ticket_type)
        }
        BaseCommands::List(options) => {
            actions::list_all_tickets(options.status, options.ticket_type)
        }
        BaseCommands::Show { id } => actions::show_ticket(id),
        BaseCommands::Edit => unimplemented!(),
        BaseCommands::Close => unimplemented!(),
        BaseCommands::Reopen => unimplemented!(),
    }?;

    Ok(())
}

mod actions;
