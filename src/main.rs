mod axum;
mod node;
mod option;
mod progress;

use std::env::args;

use axum::Axum;
use node::Node;
use option::Option;
use tokio::fs::write;

#[tokio::main]
async fn main() {
    let option = if args().len() == 2 {
        Option {
            project_name: args().nth(1).unwrap(),
        }
    } else {
        Option::read_from_term().unwrap()
    };
    Node::new(&option.project_name)
        .create_project()
        .await
        .unwrap();
    Axum::new(option.project_name.clone())
        .create_project()
        .await
        .unwrap();

    // create Taskfile.yaml
    const TASKFILE_TEXT: &str = include_str!("../template/Taskfile.yaml");
    write(
        format!("{}/{}", &option.project_name, "Taskfile.yaml"),
        TASKFILE_TEXT.replace("{{PROJECT_NAME}}", &option.project_name),
    )
    .await
    .unwrap();

    // create .dockerignore
    const DOCKERIGNORE_TEXT: &str = include_str!("../template/.dockerignore");
    write(
        format!("{}/{}", &option.project_name, ".dockerignore"),
        DOCKERIGNORE_TEXT,
    )
    .await
    .unwrap();

    // create compose.yml
    const COMPOSE_YML_TEXT: &str = include_str!("../template/compose.yml");
    write(
        format!("{}/{}", &option.project_name, "compose.yml"),
        COMPOSE_YML_TEXT,
    )
    .await
    .unwrap();

    // create Dockerfile
    const DOCKERFILE_TEXT: &str = include_str!("../template/Dockerfile");
    write(
        format!("{}/{}", &option.project_name, "Dockerfile"),
        DOCKERFILE_TEXT.replace("{{PROJECT_NAME}}", &option.project_name),
    )
    .await
    .unwrap();

    // create Dockerfile.server
    const DOCKERFILE_SERVER_TEXT: &str = include_str!("../template/Dockerfile.server");
    write(
        format!("{}/{}", &option.project_name, "Dockerfile.server"),
        DOCKERFILE_SERVER_TEXT,
    )
    .await
    .unwrap();

    // create Dockerfile.web
    const DOCKERFILE_WEB_TEXT: &str = include_str!("../template/Dockerfile.web");
    write(
        format!("{}/{}", &option.project_name, "Dockerfile.web"),
        DOCKERFILE_WEB_TEXT,
    )
    .await
    .unwrap();
}
