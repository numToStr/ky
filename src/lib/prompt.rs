use super::KyError;
use dialoguer::Input;

pub struct Prompt;

type PromptReturn = Result<Option<String>, KyError>;

impl Prompt {
    fn prompt(title: &str) -> PromptReturn {
        let input: String = Input::new()
            .with_prompt(title)
            .allow_empty(true)
            .interact_text()?;

        let new_input = match input.as_ref() {
            "" => None,
            x => Some(x.to_string()),
        };

        Ok(new_input)
    }

    pub fn username() -> PromptReturn {
        Self::prompt("Username")
    }

    pub fn url() -> PromptReturn {
        Self::prompt("URL")
    }

    pub fn expires() -> PromptReturn {
        Self::prompt("Expires (dd/mm/yyyy)")
    }

    pub fn notes() -> PromptReturn {
        Self::prompt("Notes")
    }
}
