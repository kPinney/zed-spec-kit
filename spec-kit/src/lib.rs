//! This crate is the main library for the Spec-Kit Zed Extension.
//!
//! It provides the core functionality for implementing the `spec-kit` development
//! framework within the Zed editor. The extension works by processing slash commands
//! from the user in the Zed AI chat and feeding structured, context-rich prompts
//! to the AI assistant for execution.

use zed_extension_api as zed;

// Declare the modules that will be part of the extension library.
mod command;
mod error;
mod handler;

// Bring the necessary structs and functions into the main library scope.
use command::{load_commands, Command};
use handler::handle_command;
use std::collections::HashMap;

/// The main struct for the Spec-Kit extension.
///
/// It holds the state for the extension, which primarily consists of the
/// collection of slash commands loaded from the bundled `.toml` files at startup.
pub struct SpecKitExtension {
    commands: HashMap<String, Command>,
}

// Register the SpecKitExtension struct as the main entry point for this extension.
zed::register_extension!(SpecKitExtension);

/// Implements the `zed::Extension` trait, which is the entry point for Zed
/// to interact with our extension.
impl zed::Extension for SpecKitExtension {
    /// This function is called once when the extension is loaded by Zed.
    ///
    /// It is responsible for initializing the extension's state by:
    /// 1. Using `include_str!` to bundle all command `.toml` definitions into the binary.
    /// 2. Calling the `load_commands` function to parse the TOML content into `Command` structs.
    /// 3. Storing the successfully loaded commands in the `SpecKitExtension` struct.
    ///
    /// If command loading fails, the extension will panic, as it cannot function
    /// without its command definitions.
    fn new() -> Self {
        let mut command_files = HashMap::new();
        command_files.insert(
            "analyze".to_string(),
            include_str!("../assets/commands/analyze.toml").to_string(),
        );
        command_files.insert(
            "clarify".to_string(),
            include_str!("../assets/commands/clarify.toml").to_string(),
        );
        command_files.insert(
            "constitution".to_string(),
            include_str!("../assets/commands/constitution.toml").to_string(),
        );
        command_files.insert(
            "context".to_string(),
            include_str!("../assets/commands/context.toml").to_string(),
        );
        command_files.insert(
            "implement".to_string(),
            include_str!("../assets/commands/implement.toml").to_string(),
        );
        command_files.insert(
            "plan".to_string(),
            include_str!("../assets/commands/plan.toml").to_string(),
        );
        command_files.insert(
            "specify".to_string(),
            include_str!("../assets/commands/specify.toml").to_string(),
        );
        command_files.insert(
            "tasks".to_string(),
            include_str!("../assets/commands/tasks.toml").to_string(),
        );

        let commands =
            load_commands(command_files).expect("Failed to load or parse command definitions");

        Self { commands }
    }

    /// This function is called by Zed every time a user invokes one of the slash
    /// commands registered by this extension in `extension.toml`.
    ///
    /// It acts as the primary dispatcher, delegating the logic to the `handle_command`
    /// function and packaging the result into the `SlashCommandOutput` format
    /// expected by Zed's AI assistant.
    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        let args_str = args.join(" ");

        let Some(worktree) = worktree else {
            return Err("This command requires an active worktree.".to_string());
        };

        match handle_command(&command.name, &args_str, &self.commands, worktree) {
            Ok(prompt) => Ok(zed::SlashCommandOutput {
                text: prompt,
                sections: vec![],
            }),
            Err(e) => Err(e.to_string()),
        }
    }
}
