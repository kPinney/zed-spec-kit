// This test file is for the `load_commands` function, which is responsible for
// parsing command definition strings (.toml format) into `Command` structs.
// Since this is part of the TDD process, the `load_commands` function does not
// yet exist, and this file is expected to fail compilation.

// We anticipate the `command` module will expose a `Command` struct and a
// `load_commands` function.
use spec_kit::command::{load_commands, Command};
use spec_kit::error::CommandError;
use std::collections::HashMap;

#[test]
fn test_load_valid_command() {
    // A map representing the bundled command files, where key is the command
    // name and value is the .toml file content as a string.
    let mut command_files = HashMap::new();
    command_files.insert(
        "specify".to_string(),
        r#"
description = "Create a new feature specification."
prompt = "Generate a spec for: $ARGUMENTS"
"#
        .to_string(),
    );

    // Call the function that we intend to create.
    let result = load_commands(command_files);

    // Assert that the function successfully parsed the command.
    assert!(result.is_ok());
    let commands = result.unwrap();

    // Verify that the parsed command is present and its fields are correct.
    assert_eq!(commands.len(), 1);
    let specify_command = commands
        .get("specify")
        .expect("Command 'specify' should be loaded");
    assert_eq!(
        specify_command.description,
        "Create a new feature specification."
    );
    assert_eq!(specify_command.prompt, "Generate a spec for: $ARGUMENTS");
}

#[test]
fn test_load_command_with_missing_field() {
    // This TOML content is invalid because it is missing the `prompt` field.
    let mut command_files = HashMap::new();
    command_files.insert(
        "invalid".to_string(),
        r#"
description = "This command is incomplete."
"#
        .to_string(),
    );

    let result = load_commands(command_files);

    // Assert that parsing fails for incomplete TOML.
    assert!(result.is_err());
}

#[test]
fn test_load_multiple_commands() {
    let mut command_files = HashMap::new();
    command_files.insert(
        "specify".to_string(),
        r#"
description = "Spec command"
prompt = "Spec prompt"
"#
        .to_string(),
    );
    command_files.insert(
        "plan".to_string(),
        r#"
description = "Plan command"
prompt = "Plan prompt"
"#
        .to_string(),
    );

    let result = load_commands(command_files);
    assert!(result.is_ok());
    let commands = result.unwrap();

    // Verify that both commands were loaded correctly.
    assert_eq!(commands.len(), 2);
    assert!(commands.contains_key("specify"));
    assert!(commands.contains_key("plan"));
    assert_eq!(commands.get("plan").unwrap().description, "Plan command");
}
