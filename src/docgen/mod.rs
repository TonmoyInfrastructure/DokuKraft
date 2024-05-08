mod docgen;
mod init;
mod introduction;
use crate::preprocess::{Preprocessor, PreprocessorContext};

pub struct DokuKraft {
    pub root: PathBuf,
    pub config: Config,
    pub docgen: Docgen,
    renderers: Vec<Box<dyn Renderer>>,
    preprocessors: Vec<Box<dyn Preprocessor>>,
}

impl DokuKraft {
    pub fn load<P: Into<PathBuf>>(docgen_root: P) -> Result<DokuKraft> {
        let docgen_root = docgen_root.into();
        let config_location = docgen_root.join("docgen.toml");
        if docgen_root.join("docgen.json").exists() {
            warn!("JSON format for configuration is not supported.");
            warn!("Use the latest TOML format as configuration.");
            warn!("To Know More:");
            warn!("\thttps://example.com/");
        }
        let mut config = if config_location.exists() {
            debug!("Loading config from {}", config_location.display());
            Config::from_disk(&config_location)?
        } else {
            Config::default()
        };
    }
}