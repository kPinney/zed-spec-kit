use crate::error::CommandError;
use serde::Deserialize;
use std::collections::HashMap;

/// Represents a single slash command definition loaded from a .toml file.
///
/// This struct holds the deserialized content from a command's `.toml` file,
/// making the description and prompt template available at runtime.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Command {
    /// A brief, user-facing description of what the command does.
    pub description: String,
    /// The master prompt template that will be processed and sent to the AI assistant.
    /// It may contain placeholders like `$ARGUMENTS`.
    pub prompt: String,
}

/// Parses a collection of command definition strings into a map of `Command` structs.
///
/// This function is the core of the command loading process. It takes the raw,
/// string-based content of the bundled .toml files and attempts to deserialize
/// each one into a valid `Command`. This allows the extension to be configured
/// entirely through its bundled TOML assets.
///
/// # Arguments
///
/// * `command_files`: A map where the key is the command name (e.g., "specify")
///   and the value is the string content of the corresponding .toml file.
///
/// # Returns
///
/// A `Result` which is either:
/// - `Ok(HashMap<String, Command>)`: A map of successfully parsed commands, keyed by name.
/// - `Err(CommandError)`: An error indicating that one of the files failed to parse.
pub fn load_commands(
    command_files: HashMap<String, String>,
) -> Result<HashMap<String, Command>, CommandError> {
    let mut commands = HashMap::new();

    for (name, content) in command_files {
        let command: Command = toml::from_str(&content)
            .map_err(|e| CommandError::TomlParseError(format!("for command '{}': {}", name, e)))?;
        commands.insert(name, command);
    }

    Ok(commands)
}
