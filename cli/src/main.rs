use chat_wizard_service::{
    init, project::Project, services::plugin::PluginService, DatabaseError, Error as ServiceError,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Exec { plugin: String },
}

#[tokio::main]
async fn main() {
    let project = Project::init().unwrap();
    let conn = init(&project.db_url).unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Exec { plugin }) => {
            let plugin_service = PluginService::new(conn.clone());
            match plugin_service.execute_by_name(plugin).await {
                Ok(_) => {}
                Err(err) => match err {
                    ServiceError::Database(err) => match err {
                        DatabaseError::NotFound => println!("Plugin {plugin} is not installed"),
                        _ => println!("{:?}", err),
                    },
                    _ => println!("{:?}", err),
                },
            }
        }
        None => {}
    }
}
