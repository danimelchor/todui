use crate::{app::App, configuration::SettingsBuilder};
use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Args {
    command: Command,
}

#[derive(Parser, Clone, Copy, ValueEnum)]
enum Command {
    /// Reset the configuration to default
    Reset,
    /// Show the configuration
    Show,
    /// Set the mode to vi
    SetViMode,
    /// Set the mode to normal
    SetNormalMode,
    /// Set the icons to special characters
    SetSpecialIcons,
    /// Set the icons to char
    SetCharIcons,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args { command } = args;

    match command {
        Command::Reset => {
            let mut sb = SettingsBuilder::default();
            sb.save_to_file()?;
            app.settings = sb.build();
        }
        Command::Show => {
            println!("{}", serde_json::to_string_pretty(&app.settings)?);
        }
        Command::SetViMode => {
            app.settings.set_vi_mode();
        }
        Command::SetNormalMode => {
            app.settings.set_normal_mode();
        }
        Command::SetSpecialIcons => {
            app.settings.set_special_icons();
        }
        Command::SetCharIcons => {
            app.settings.set_char_icons();
        }
    }
    Ok(())
}
