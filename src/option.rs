use anyhow::Result;
use dialoguer::Input;

pub struct Option {
    pub project_name: String,
}

impl Option {
    pub fn read_from_term() -> Result<Self> {
        let project_name: String = Input::new().with_prompt("Project name").interact_text()?;
        Ok(Option { project_name })
    }
}
