use std::{path::Path, time::Duration};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{
    fs::{create_dir_all, read_to_string, remove_dir_all, remove_file, rename, write},
    process::Command,
};

use crate::bar;

struct NpmPackage {
    name: String,
    dev: bool,
}

impl NpmPackage {
    pub fn new(name: &str, dev: bool) -> Self {
        Self {
            name: name.to_string(),
            dev,
        }
    }

    pub fn to_install_command_args(&self) -> Vec<&str> {
        let mut args = vec![self.name.as_str()];
        if self.dev {
            args.push("-D");
        }
        args
    }
}

pub struct Node {
    pub project_name: String,
}

impl Node {
    pub fn new(project_name: &str) -> Self {
        Self {
            project_name: project_name.to_string(),
        }
    }

    pub async fn create_project(&self) -> Result<()> {
        // create farm project
        let bar = bar!("Creating farm project...", 1);
        Command::new("pnpm")
            .args([
                "create",
                "farm@latest",
                &self.project_name,
                "--template",
                "react",
            ])
            .output()
            .await?;
        bar.finish();

        // rename src/ to page/
        let project_path = Path::new(&self.project_name);
        rename(
            format!("{}/src", &self.project_name),
            format!("{}/page", &self.project_name),
        )
        .await?;
        let tsconfig_path = project_path.join("tsconfig.json");
        let tsconfig_path = Path::new(&tsconfig_path);
        let tsconfig_text = read_to_string(tsconfig_path)
            .await?
            .replace(r#""include": ["src"],"#, r#""include": ["page", "tests"],"#);
        write(tsconfig_path, tsconfig_text).await?;
        const FARM_CONFIG_TEXT: &str = include_str!("../template/farm.config.ts");
        write(project_path.join("farm.config.ts"), FARM_CONFIG_TEXT).await?;

        // remove default files
        remove_dir_all(project_path.join("public")).await?;
        remove_dir_all(project_path.join("page/assets")).await?;
        remove_file(project_path.join("index.html")).await?;
        remove_file(project_path.join("page/index.css")).await?;
        remove_file(project_path.join("page/main.css")).await?;
        remove_file(project_path.join("page/typings.d.ts")).await?;

        // set default index.tsx, main.tsx, index.html, e2e.test.ts
        const INDEX_TSX_TEXT: &str = include_str!("../template/index.tsx");
        write(project_path.join("page/index.tsx"), INDEX_TSX_TEXT).await?;
        const MAIN_TSX_TEXT: &str = include_str!("../template/main.tsx");
        write(project_path.join("page/main.tsx"), MAIN_TSX_TEXT).await?;
        const INDEX_HTML_TEXT: &str = include_str!("../template/index.html");
        write(project_path.join("page/index.html"), INDEX_HTML_TEXT).await?;
        const E2E_TEST_TEXT: &str = include_str!("../template/e2e.test.ts");
        create_dir_all(project_path.join("tests")).await?;
        write(project_path.join("tests/e2e.test.ts"), E2E_TEST_TEXT).await?;

        // install dependencies
        let bar = bar!("Installing dependencies...", 2);
        Command::new("pnpm")
            .arg("install")
            .current_dir(&self.project_name)
            .output()
            .await?;
        bar.finish();

        // add dependencies
        let bar = bar!("Adding dependencies...", 3);
        let dependencies = vec![
            NpmPackage::new("@biomejs/biome", true),
            NpmPackage::new("@playwright/test", true),
        ];
        for dependency in dependencies {
            let mut args = vec!["install"];
            args.extend(dependency.to_install_command_args());
            Command::new("pnpm")
                .args(&args)
                .current_dir(&self.project_name)
                .output()
                .await?;
        }
        bar.finish();

        // biome init
        let bar = bar!("Initializing biome...", 4);
        let response = Command::new("pnpm")
            .args(["biome", "init"])
            .current_dir(&self.project_name)
            .output()
            .await?;
        if !response.status.success() {
            anyhow::bail!("Failed to initialize biome");
        }
        bar.finish();

        // add biome and playwright scripts
        let package_json_path = project_path.join("package.json");
        let package_json_path = Path::new(&package_json_path);
        let package_json_text = read_to_string(package_json_path).await?;
        let package_json_text = package_json_text.replace(
            r#""scripts": {"#,
            r#""scripts": {
    "check": "biome check ./page",
    "check:write": "biome check --write --unsafe ./page",
    "e2e": "playwright test","#,
        );
        write(package_json_path, package_json_text).await?;

        // run biome check:write
        let bar = bar!("Linting project...", 5);
        let response = Command::new("pnpm")
            .args(["run", "check:write"])
            .current_dir(&self.project_name)
            .output()
            .await?;
        if !response.status.success() {
            anyhow::bail!(
                "Failed to lint project: {}",
                String::from_utf8_lossy(&response.stderr)
            );
        }
        bar.finish();

        // install playwight
        let bar = bar!("Installing playwright...", 6);
        Command::new("pnpm")
            .arg("run")
            .arg("playwright:install")
            .arg("--with-deps")
            .current_dir(&self.project_name)
            .output()
            .await?;
        bar.finish();

        // check if the project is created successfully
        let bar = bar!("Checking project...", 7);
        let response = Command::new("pnpm")
            .args(["run", "check"])
            .current_dir(&self.project_name)
            .output()
            .await?;
        if !response.status.success() {
            anyhow::bail!("Failed to check project");
        }
        bar.finish();

        Ok(())
    }
}
