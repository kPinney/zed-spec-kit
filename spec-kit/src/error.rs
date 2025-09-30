use std::fmt;

/// Represents all possible errors that can occur within the Spec-Kit extension.
#[derive(Debug)]
#[allow(dead_code)]
pub enum CommandError {
    /// Error indicating that a requested slash command was not found in the loaded definitions.
    /// Contains the name of the command that was not found.
    CommandNotFound(String),
    /// Error indicating that a command requiring user input was run without any.
    /// Contains the name of the command that was missing an argument.
    MissingArgument(String),
    /// Error indicating that a file required by a command does not exist.
    /// Contains the path to the file that was not found.
    PrerequisiteFileNotFound(String),
    /// Error indicating that a command's .toml definition file is malformed.
    /// Contains a detailed error message from the TOML parser.
    TomlParseError(String),
    /// Represents a generic I/O error that may occur.
    /// Contains a detailed error message.
    IoError(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::CommandNotFound(command) => write!(f, "Command not found: /{}", command),
            CommandError::MissingArgument(command) => {
                write!(f, "Missing argument for command: /{}", command)
            }
            CommandError::PrerequisiteFileNotFound(path) => {
                write!(f, "Prerequisite file not found: {}", path)
            }
            CommandError::TomlParseError(details) => {
                write!(f, "Failed to parse TOML file: {}", details)
            }
            CommandError::IoError(details) => write!(f, "I/O Error: {}", details),
        }
    }
}

impl std::error::Error for CommandError {}
