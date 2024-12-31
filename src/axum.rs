use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::{path::Path, time::Duration};
use tokio::{fs::write, process::Command};

use crate::bar;

pub struct Axum {
    pub project_name: String,
}

impl Axum {
    pub fn new(project_name: String) -> Self {
        Self { project_name }
    }

    pub async fn create_project(&self) -> Result<()> {
        let project_path = Path::new(&self.project_name);

        // init axum project
        let bar = bar!("Creating axum project...", 9);
        let response = Command::new("cargo")
            .arg("init")
            .current_dir(project_path)
            .output()
            .await?;
        const MAIN_TEXT: &str = include_str!("../template/main.rs");
        write(project_path.join("src/main.rs"), MAIN_TEXT).await?;
        bar.finish();
        if !response.status.success() {
            anyhow::bail!(
                "Failed to create axum project: {}",
                String::from_utf8_lossy(&response.stderr)
            );
        }

        // install common dependencies
        let bar = bar!("Installing dependencies...", 10);
        let response = Command::new("cargo")
            .args([
                "add",
                "anyhow",
                "axum",
                "axum-test",
                "tracing",
                "tracing-subscriber",
            ])
            .current_dir(project_path)
            .output()
            .await?;
        if !response.status.success() {
            anyhow::bail!(
                "Failed to install dependencies: {}",
                String::from_utf8_lossy(&response.stderr)
            );
        }

        // install tokio
        let response = Command::new("cargo")
            .args(["add", "tokio", "-F", "full"])
            .current_dir(project_path)
            .output()
            .await?;
        if !response.status.success() {
            anyhow::bail!(
                "Failed to install dependencies: {}",
                String::from_utf8_lossy(&response.stderr)
            );
        }

        // install tower-http
        let response = Command::new("cargo")
            .args(["add", "tower-http", "-F", "fs,trace"])
            .current_dir(project_path)
            .output()
            .await?;
        if !response.status.success() {
            anyhow::bail!(
                "Failed to install dependencies: {}",
                String::from_utf8_lossy(&response.stderr)
            );
        }
        bar.finish();

        Ok(())
    }
}
