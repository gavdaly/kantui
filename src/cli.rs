use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, after_help = "If no command is given, a TUI interface will be launched")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    #[command(subcommand)]
    #[command(about = "Card related commands")]
    Card(CardCommands),
    #[command(subcommand)]
    #[command(about = "Column related commands")]
    Column(ColumnCommands),
    #[command(about = "List all columns")]
    List,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ColumnCommands {
    #[command(about = "Add a new column")]
    Add { title: String },
    #[command(about = "List cards in a column")]
    List { id: String },
    #[command(about = "Remove a column")]
    Remove { id: String },
    #[command(about = "Rename a column")]
    Rename { id: String, title: String },
}

#[derive(Subcommand, Clone, Debug)]
pub enum CardCommands {
    #[command(about = "Add a new card to a column")]
    Add { title: String, column: String },
    #[command(about = "Remove a card")]
    Remove { id: String },
    #[command(about = "Rename a card")]
    Rename { id: String, title: String },
    #[command(about = "Move a card to a different column")]
    Move { id: String, column: String },
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    dbg!(&cli);

    match &cli.command {
        Some(Command::List) => {
            println!("List");
        }
        Some(Command::Column(command)) => match command {
            ColumnCommands::Add { title } => {
                println!("Add column with title: {title}");
            }
            ColumnCommands::List { id } => {
                println!("List column: {id}");
            }
            ColumnCommands::Remove { id } => {
                println!("Remove column: {id}");
            }
            ColumnCommands::Rename { id, title } => {
                println!("Rename column id: {id} to title: {title}");
            }
        },
        Some(Command::Card(command)) => match command {
            CardCommands::Add { column, title } => {
                println!("Add card to column: {column} with title: {title}");
            }
            CardCommands::Remove { id } => {
                println!("Remove card: {id}");
            }
            CardCommands::Rename { id, title } => {
                println!("Rename card id: {id} to title: {title}");
            }
            CardCommands::Move { id, column } => {
                println!("Move card: {id} to column: {column}");
            }
        },
        None => {
            println!("Will launch future tui")
        }
    }

    Ok(())
}
