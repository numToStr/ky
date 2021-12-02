use clap::Parser;

#[derive(Debug, Parser)]
pub enum Shell {
    /// Bourne Again SHell (bash)
    Bash,

    /// Elvish shell
    Elvish,

    /// Friendly Interactive SHell (fish)
    Fish,

    /// Windows PowerShell
    #[clap(name = "pwsh")]
    PowerShell,

    /// Z SHell (zsh)
    Zsh,
}
