mod project;
mod utils;

use chat_wizard_api::app;
use project::Project;

#[tokio::main]
async fn main() {
    env_logger::init();
    let project = Project::init().await.unwrap();
    let conn = chat_wizard_service::init(&project.db_url).unwrap();

    let port = 23333;

    app(port, conn).await;
}
