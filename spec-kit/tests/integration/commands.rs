// Updated integration tests for the Spec-Kit Zed Extension.
// These tests verify the end-to-end behavior of the slash commands,
// ensuring they correctly process input and return the expected prompts
// or errors, according to the new `run_slash_command` API.

use spec_kit::SpecKitExtension;
use zed_extension_api::{self as zed, Extension};

/// Tests the happy path for the `/specify` command, which requires an argument.
/// It verifies that the user's input is correctly substituted into the prompt template.
#[zed::test]
fn test_specify_command_with_argument() {
    // Arrange: Initialize the extension, which loads all command definitions.
    let mut extension = SpecKitExtension::new();
    let command = zed::SlashCommand {
        name: "specify".to_string(),
    };
    let user_input = "a new login feature";
    let args = user_input.split_whitespace().map(String::from).collect();
    let workspace = zed::test::test_workspace();
    let worktree = Some(&workspace);

    // Act: Run the command.
    let output = extension
        .run_slash_command(command, args, worktree)
        .expect("Command execution should succeed");

    // Assert: The output text should contain the user's input, proving that
    // the `$ARGUMENTS` placeholder was correctly replaced.
    assert!(
        output.text.contains(user_input),
        "The output prompt for /specify did not contain the user's arguments."
    );
}

/// Tests a command that does not require an argument, like `/clarify`.
/// It verifies that the correct prompt is returned without any substitution.
#[zed::test]
fn test_command_without_argument() {
    // Arrange
    let mut extension = SpecKitExtension::new();
    let command = zed::SlashCommand {
        name: "clarify".to_string(),
    };
    let args = Vec::new(); // No arguments provided.
    let workspace = zed::test::test_workspace();
    let worktree = Some(&workspace);

    // Act
    let output = extension
        .run_slash_command(command, args, worktree)
        .expect("Command execution should succeed");

    // Assert: The output should contain a known snippet from the clarify.prompt.md,
    // ensuring the correct prompt was loaded and returned.
    assert!(
        output.text.contains("Identify underspecified areas"),
        "The output prompt for /clarify seems incorrect."
    );
}

/// Tests the error handling for a command that requires an argument but receives none.
#[zed::test]
fn test_command_missing_required_argument() {
    // Arrange
    let mut extension = SpecKitExtension::new();
    let command = zed::SlashCommand {
        name: "specify".to_string(),
    };
    let args = Vec::new(); // Missing the required argument.
    let workspace = zed::test::test_workspace();
    let worktree = Some(&workspace);

    // Act: Run the command.
    let result = extension.run_slash_command(command, args, worktree);

    // Assert: The command should return an error.
    assert!(
        result.is_err(),
        "Expected command to fail, but it succeeded."
    );
    let error_message = result.unwrap_err();
    assert!(
        error_message.contains("Missing argument for command: /specify"),
        "The error message for missing argument is incorrect."
    );
}

/// Tests the error handling for a non-existent command.
#[zed::test]
fn test_unknown_command() {
    // Arrange
    let mut extension = SpecKitExtension::new();
    let command = zed::SlashCommand {
        name: "nonexistent".to_string(),
    };
    let args = Vec::new();
    let workspace = zed::test::test_workspace();
    let worktree = Some(&workspace);

    // Act
    let result = extension.run_slash_command(command, args, worktree);

    // Assert
    assert!(
        result.is_err(),
        "Expected command to fail, but it succeeded."
    );
    let error_message = result.unwrap_err();
    assert!(
        error_message.contains("Command not found: /nonexistent"),
        "The error message for an unknown command is incorrect."
    );
}
