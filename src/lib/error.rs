use thiserror::Error;

#[derive(Debug, Error)]
pub enum LokError {
    // Generic and Unknown error, that I don't want to handle
    #[error("Something went wrong: `{0}")]
    Any(String),

    // #startregion: Errors related to database
    #[error("Unable to establish database connection")]
    Connection,

    #[error("Value not found for key: `{0}`")]
    NotFound(&'static str),

    #[error("Unable to get the value for `{0}`")]
    Get(&'static str),

    #[error("Unable to set the value for `{0}`")]
    Set(&'static str),

    #[error("Unable to delete the value for `{0}`")]
    Delete(&'static str),
    // #endregion
}
