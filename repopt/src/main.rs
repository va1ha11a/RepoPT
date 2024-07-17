mod actions;
mod in_repo_db;
mod output_formatter;

use std::path::PathBuf;

use clap::Parser;
use in_repo_db::structs::{TicketStatus, TicketType};

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

mod config {
    use std::{path::PathBuf, sync::OnceLock};

    use crate::output_formatter::OutputFormatter;

    #[derive(Debug)]
    pub(crate) struct Config {
        pub formatter: OutputFormatter,
        pub irdb_path: PathBuf,
    }

    pub(crate) static CONFIG: OnceLock<Config> = OnceLock::new();
}

#[derive(Parser, Debug)]
#[command(name = "RepoRT", about = "CLI for RepoRT: In Repo Ticketing System")]
struct Cli {
    #[arg(long, default_value = "json", global = true)]
    format: output_formatter::OutputFormatter,

    #[arg(long, value_parser = clap::value_parser!(PathBuf), global = true, default_value = ".irdb")]
    irdb_path: PathBuf,

    #[command(subcommand)]
    base_command: BaseCommands,
}

#[derive(Parser, Debug)]
enum BaseCommands {
    #[clap(name = "init", about = "Initialize a new repository")]
    Init,
    #[clap(name = "add", about = "Add a new item")]
    #[command(subcommand)]
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
enum AddOptions {
    #[clap(name = "ticket", about = "Add a new ticket")]
    Ticket(AddTicketOptions),
    #[clap(name = "project", about = "Add a new project")]
    Project(AddProjectOptions),
}

#[derive(Parser, Debug)]
struct AddProjectOptions {
    #[clap(long, value_enum)]
    name: Option<String>,
    #[clap(long, value_enum)]
    description: Option<String>,
}

#[derive(Parser, Debug)]
struct AddTicketOptions {
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
    let cli = Cli::parse();

    let _ = config::CONFIG.set(config::Config {
        formatter: cli.format,
        irdb_path: cli.irdb_path,
    });

    match cli.base_command {
        BaseCommands::Init => actions::init_new_repository(),
        BaseCommands::Add(AddOptions::Ticket(ticket_options)) => actions::add_new_ticket(
            ticket_options.title,
            ticket_options.status,
            ticket_options.ticket_type,
        ),
        BaseCommands::Add(AddOptions::Project(project_options)) => {
            actions::add_new_project(project_options.name, project_options.description)
        }
        BaseCommands::List(options) => {
            actions::list_all_tickets(options.status, options.ticket_type)
        }
        BaseCommands::Show { id } => actions::show_ticket(id),
        BaseCommands::Edit => unimplemented!(),
        BaseCommands::Close => actions::close_ticket(),
        BaseCommands::Reopen => actions::reopen_ticket(),
    }?;

    Ok(())
}
