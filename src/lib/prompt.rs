use super::KyError;
use dialoguer::{
    console::{style, Style},
    theme::{ColorfulTheme, Theme},
    Confirm, Input,
};

pub const PREFIX: char = '~';

type PromptReturn = Result<Option<String>, KyError>;

pub struct Prompt;

impl Prompt {
    pub fn theme() -> ColorfulTheme {
        ColorfulTheme {
            prompt_prefix: style(PREFIX.to_string())
                .for_stderr()
                .black()
                .bright()
                .bold(),
            success_prefix: style(PREFIX.to_string()).for_stderr().bold(),
            error_prefix: style(PREFIX.to_string()).for_stderr().red(),
            values_style: Style::new().for_stderr().yellow(),
            prompt_style: Style::new().for_stderr(),
            ..ColorfulTheme::default()
        }
    }

    fn prompt_with_default(title: &str, theme: &impl Theme, default: String) -> PromptReturn {
        let input: String = Input::with_theme(theme)
            .default(default)
            .with_prompt(title)
            .allow_empty(true)
            .interact_text()?;

        let new_input = match input.as_ref() {
            "" => None,
            x => Some(x.to_string()),
        };

        Ok(new_input)
    }
    fn prompt(title: &str, theme: &impl Theme) -> PromptReturn {
        let input: String = Input::with_theme(theme)
            .with_prompt(title)
            .allow_empty(true)
            .interact_text()?;

        let new_input = match input.as_ref() {
            "" => None,
            x => Some(x.to_string()),
        };

        Ok(new_input)
    }

    pub fn confirm(theme: &impl Theme) -> std::io::Result<bool> {
        Confirm::with_theme(theme)
            .with_prompt("Are you sure you want to proceed?")
            .default(false)
            .wait_for_newline(true)
            .interact()
    }

    pub fn username(theme: &impl Theme) -> PromptReturn {
        Self::prompt("Username", theme)
    }

    pub fn url(theme: &impl Theme) -> PromptReturn {
        Self::prompt("URL", theme)
    }

    pub fn expires(theme: &impl Theme) -> PromptReturn {
        Self::prompt("Expires (dd/mm/yyyy)", theme)
    }

    pub fn notes(theme: &impl Theme) -> PromptReturn {
        Self::prompt("Notes", theme)
    }

    pub fn username_with_default(theme: &impl Theme, default: String) -> PromptReturn {
        Self::prompt_with_default("Username", theme, default)
    }

    pub fn url_with_default(theme: &impl Theme, default: String) -> PromptReturn {
        Self::prompt_with_default("URL", theme, default)
    }

    pub fn expires_with_default(theme: &impl Theme, default: String) -> PromptReturn {
        Self::prompt_with_default("Expires", theme, default)
    }

    pub fn notes_with_default(theme: &impl Theme, default: String) -> PromptReturn {
        Self::prompt_with_default("Notes", theme, default)
    }
}
