use crate::command::Command;
use crate::error::CommandError;
use std::collections::HashMap;
use zed_extension_api as zed;

/// Handles the dispatch of a single slash command.
///
/// This function is the core of the command execution logic. It takes the invoked
/// command name and its arguments, looks up the corresponding `Command` in the
/// provided map, and then processes its prompt template to generate the final
/// output for the AI assistant.
///
/// # Arguments
///
/// * `name`: The name of the command being invoked (e.g., "specify").
/// * `args`: The string of arguments provided by the user after the command name.
/// * `commands`: A map containing all the loaded commands for the extension.
/// * `worktree`: The Zed worktree context, used to get project-specific information.
///
/// # Returns
///
/// A `Result` containing either:
/// - `Ok(String)`: The fully processed and formatted prompt to be sent to the AI.
/// - `Err(CommandError)`: An error indicating what went wrong, such as the
///   command not being found or a missing argument.
pub fn handle_command(
    name: &str,
    args: &str,
    commands: &HashMap<String, Command>,
    worktree: &zed::Worktree,
) -> Result<String, CommandError> {
    // 1. Find the command by its name in the map of loaded commands.
    let command = commands
        .get(name)
        .ok_or_else(|| CommandError::CommandNotFound(name.to_string()))?;

    // 2. Enforce rules for commands that require arguments.
    let commands_requiring_args = ["specify", "constitution"];
    if commands_requiring_args.contains(&name) && args.is_empty() {
        return Err(CommandError::MissingArgument(name.to_string()));
    }

    // 3. Process the prompt template.
    // Replace the `$ARGUMENTS` placeholder with the user-provided arguments.
    let mut processed_prompt = command.prompt.replace("$ARGUMENTS", args);

    // 4. Dynamically inject the project root path if the placeholder exists.
    // This ensures the returned prompt is never identical to the raw template,
    // which appears to be a condition for the Zed Agent to accept the output.
    let project_root = worktree.root_path();
    processed_prompt = processed_prompt.replace("$PROJECT_ROOT", &project_root);

    // 5. Return the processed prompt.
    Ok(processed_prompt)
}
