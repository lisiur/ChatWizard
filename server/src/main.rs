mod project;
mod utils;

use chat_wizard_api::app;
use clap::Parser;
use project::Project;

#[derive(Parser, Debug)]
#[command(name = "chat-wizard")]
#[command(author = "Lisiur Day <lisiurday@gmail.com>")]
#[command(about = "Chat Wizard Server", long_about = None)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value = "23333")]
    port: u16,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    let project = Project::init().await.unwrap();
    let conn = chat_wizard_service::init(&project.db_url).unwrap();

    let port = args.port;

    app(port, conn).await;
}
