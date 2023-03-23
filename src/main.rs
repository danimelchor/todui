use anyhow::Result;
use todui::configuration::get_configuration;
use todui::{app::App, cli, ui};

fn main() -> Result<()> {
    // Check args, if none, run ui, else run cli
    let settings = get_configuration();
    let app = App::new(settings);

    if std::env::args().len() > 1 {
        cli::start_cli(app)
    } else {
        ui::start_ui(app)
    }
}
