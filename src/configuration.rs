use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub store_file: String,
}

pub fn get_config_path() -> String {
    let release = "/Users/danielmelchor/.config/rust-todo/configuration.yml".to_string();
    let debug = "./configuration.yml".to_string();

    #[cfg(debug_assertions)]
    return debug.to_string();

    #[cfg(not(debug_assertions))]
    return release.to_string();
}

pub fn get_configuration() -> Settings {
    config::Config::builder()
        .add_source(config::File::new(
            get_config_path().as_str(),
            config::FileFormat::Yaml,
        ))
        .build()
        .unwrap()
        .try_deserialize()
        .expect("Could not deserialize configuration")
}
