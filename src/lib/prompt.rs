use super::KyResult;
use dialoguer::{
    console::{style, Style},
    theme::{ColorfulTheme, Theme},
    Confirm, Input,
};
use std::io;

pub const PREFIX: char = '~';
pub const EMPTY: &str = "-";

type PromptReturn = KyResult<String>;

pub struct Prompt;

impl Prompt {
    #[inline]
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

    #[inline]
    fn prompt_with_default(title: &str, theme: &impl Theme, default: String) -> PromptReturn {
        let input: String = Input::with_theme(theme)
            .default(default)
            .with_prompt(title)
            .allow_empty(true)
            .interact_text()?;

        let new_input = match input.as_str() {
            EMPTY => String::default(),
            _ => input,
        };

        Ok(new_input)
    }

    #[inline]
    fn prompt(title: &str, theme: &impl Theme) -> PromptReturn {
        let input: String = Input::with_theme(theme)
            .with_prompt(title)
            .allow_empty(true)
            .interact_text()?;

        let new_input = match input.as_str() {
            EMPTY => String::default(),
            _ => input,
        };

        Ok(new_input)
    }

    #[inline]
    pub fn confirm(title: &str, theme: &impl Theme) -> io::Result<bool> {
        Confirm::with_theme(theme)
            .with_prompt(title)
            .default(false)
            .wait_for_newline(true)
            .interact()
    }

    #[inline]
    pub fn proceed(theme: &impl Theme) -> io::Result<bool> {
        Self::confirm("Are you sure you want to proceed?", theme)
    }

    #[inline]
    pub fn backup_exist(theme: &impl Theme) -> io::Result<bool> {
        Self::confirm("Backup already exists. Do you want to proceed?", theme)
    }

    #[inline]
    pub fn export_exist(theme: &impl Theme) -> io::Result<bool> {
        Self::confirm("Export already exists. Do you want to proceed?", theme)
    }

    #[inline]
    pub fn vault_exist(theme: &impl Theme) -> io::Result<bool> {
        Self::confirm("Vault already exists. Do you want to proceed?", theme)
    }

    #[inline]
    pub fn username(theme: &impl Theme) -> PromptReturn {
        Self::prompt("Username", theme)
    }

    #[inline]
    pub fn website(theme: &impl Theme) -> PromptReturn {
        Self::prompt("Website", theme)
    }

    #[inline]
    pub fn expires(theme: &impl Theme) -> PromptReturn {
        Self::prompt("Expires (dd/mm/yyyy)", theme)
    }

    #[inline]
    pub fn notes(theme: &impl Theme) -> PromptReturn {
        Self::prompt("Notes", theme)
    }

    #[inline]
    pub fn username_with_default(theme: &impl Theme, default: String) -> PromptReturn {
        Self::prompt_with_default("Username", theme, default)
    }

    #[inline]
    pub fn website_with_default(theme: &impl Theme, default: String) -> PromptReturn {
        Self::prompt_with_default("Website", theme, default)
    }

    #[inline]
    pub fn expires_with_default(theme: &impl Theme, default: String) -> PromptReturn {
        Self::prompt_with_default("Expires", theme, default)
    }

    #[inline]
    pub fn notes_with_default(theme: &impl Theme, default: String) -> PromptReturn {
        Self::prompt_with_default("Notes", theme, default)
    }
}
