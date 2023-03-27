use crate::{app::App, configuration::SettingsBuilder};
use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Args {
    /// Reset the configuration to default
    #[clap(long)]
    reset: bool,
    /// Show the configuration
    #[clap(long)]
    show: bool,
    /// Set the keybindings mode
    #[clap(long)]
    mode: Option<Mode>,
    /// Set the icons
    #[clap(long)]
    icons: Option<Icons>,
}

#[derive(Parser, Clone, Copy, ValueEnum)]
enum Icons {
    /// Set the icons to special characters
    Special,
    /// Set the icons to char
    Chars,
}

#[derive(Parser, Clone, Copy, ValueEnum)]
enum Mode {
    /// Set the mode to vi
    Vi,
    /// Set the mode to normal
    Normal,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args {
        reset,
        show,
        mode,
        icons,
    } = args;

    if reset {
        let mut sb = SettingsBuilder::default();
        sb.save_to_file()?;
        app.settings = sb.build();
    }

    match mode {
        Some(Mode::Vi) => app.settings.set_vi_mode(),
        Some(Mode::Normal) => app.settings.set_normal_mode(),
        None => {}
    }

    match icons {
        Some(Icons::Special) => app.settings.set_special_icons(),
        Some(Icons::Chars) => app.settings.set_char_icons(),
        None => {}
    }

    if show {
        println!("{}", serde_json::to_string_pretty(&app.settings)?);
    }

    Ok(())
}
