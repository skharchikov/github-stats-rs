mod telemetry;

pub use telemetry::*;

use confique::Config;
use dotenvy::dotenv;
use secrecy::SecretString;

#[derive(Debug, Config, Clone)]
pub struct Configuration {
    #[config(env = "ACCESS_TOKEN")]
    access_token: SecretString,
    #[config(env = "GITHUB_ACTOR")]
    github_actor: String,
    #[config(env = "EXCLUDED", default = "")]
    excluded_repos: String,
    #[config(env = "EXCLUDED_LANGS", default = "")]
    excluded_langs: String,
    #[config(env = "EXCLUDE_FORKED_REPOS", default = false)]
    exclude_forked_repos: bool,
    #[config(env = "TEMPLATE_FOLDER", default = "resources/templates")]
    template_folder: String,
    #[config(env = "OUTPUT_FOLDER", default = "resources/generated")]
    output_folder: String,
}

impl Configuration {
    pub fn access_token(&self) -> &SecretString {
        &self.access_token
    }

    pub fn github_actor(&self) -> &str {
        &self.github_actor
    }

    pub fn excluded_repos(&self) -> Vec<&str> {
        self.excluded_repos.split(',').collect()
    }

    pub fn excluded_langs(&self) -> Vec<&str> {
        self.excluded_langs.split(',').collect()
    }

    pub fn exclude_forked_repos(&self) -> bool {
        self.exclude_forked_repos
    }

    pub fn load_or_die() -> Self {
        dotenv()
            .map(|_| tracing::warn!("Variables used are being loaded from .env file"))
            .map_err(|_| tracing::info!("No .env file found, using environment variables"))
            .ok();

        Configuration::builder()
            .env()
            .load()
            .expect("Failed to load configuration")
    }

    pub fn template_folder(&self) -> &str {
        &self.template_folder
    }

    pub fn output_folder(&self) -> &str {
        &self.output_folder
    }
}
