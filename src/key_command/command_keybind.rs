use crate::fs::FileType;
use crate::{config::clean::keymap::KeyMapping, key_command::Command};
use std::collections::HashMap;

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind {
        commands: HashMap<Option<FileType>, Vec<Command>>,
        description: Option<String>,
    },
    CompositeKeybind(KeyMapping),
}

impl std::fmt::Display for CommandKeybind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandKeybind::SimpleKeybind {
                commands: _,
                description: Some(desc),
            } => write!(f, "{}", desc),
            CommandKeybind::SimpleKeybind {
                commands,
                description: None,
            } => {
                for (filetype, cmd) in commands {
                    let filetype = match filetype {
                        None => "",
                        Some(FileType::Directory) => " [Directory]",
                        Some(FileType::File) => " [File]",
                    };
                    write!(f, "{}{}, ", cmd[0], filetype)?;
                }
                Ok(())
            }
            CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
        }
    }
}
