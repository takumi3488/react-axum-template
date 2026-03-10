mod axum;
mod node;
mod option;
mod progress;

use std::env::args;

use axum::Axum;
use node::Node;
use option::Option;
use tokio::fs::write;

const TASKFILE_TEXT: &str = include_str!("../template/Taskfile.yaml");
const DOCKERIGNORE_TEXT: &str = include_str!("../template/.dockerignore");
const COMPOSE_YML_TEXT: &str = include_str!("../template/compose.yml");
const DOCKERFILE_TEXT: &str = include_str!("../template/Dockerfile");
const DOCKERFILE_SERVER_TEXT: &str = include_str!("../template/Dockerfile.server");
const DOCKERFILE_WEB_TEXT: &str = include_str!("../template/Dockerfile.web");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let option = if args().len() == 2 {
        Option {
            project_name: args()
                .nth(1)
                .ok_or_else(|| anyhow::anyhow!("Project name is required"))?,
        }
    } else {
        Option::read_from_term()?
    };
    Node::new(&option.project_name)
        .create_project()
        .await?;
    Axum::new(option.project_name.clone())
        .create_project()
        .await?;

    // create Taskfile.yaml
    write(
        format!("{}/{}", &option.project_name, "Taskfile.yaml"),
        TASKFILE_TEXT.replace("{{PROJECT_NAME}}", &option.project_name),
    )
    .await?;

    // create .dockerignore
    write(
        format!("{}/{}", &option.project_name, ".dockerignore"),
        DOCKERIGNORE_TEXT,
    )
    .await?;

    // create compose.yml
    write(
        format!("{}/{}", &option.project_name, "compose.yml"),
        COMPOSE_YML_TEXT,
    )
    .await?;

    // create Dockerfile
    write(
        format!("{}/{}", &option.project_name, "Dockerfile"),
        DOCKERFILE_TEXT.replace("{{PROJECT_NAME}}", &option.project_name),
    )
    .await?;

    // create Dockerfile.server
    write(
        format!("{}/{}", &option.project_name, "Dockerfile.server"),
        DOCKERFILE_SERVER_TEXT,
    )
    .await?;

    // create Dockerfile.web
    write(
        format!("{}/{}", &option.project_name, "Dockerfile.web"),
        DOCKERFILE_WEB_TEXT,
    )
    .await?;

    Ok(())
}
