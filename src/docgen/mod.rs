mod docgen;
mod init;
mod introduction;
use crate::preprocess::{Preprocessor, PreprocessorContext};
use crate::renderer::{CmdRenderer, HtmlHandlebars, MarkdownRenderer, RenderContext, Renderer};

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
        if let Some(html_config) = config.html_config() {
            if html_config.google_analytics.is_some() {
                warn!(
                    "The output.html.google-analytics field has been deprecated; \
                     it will be removed in a future release.\n\
                     Consider placing the appropriate site tag code into the \
                     theme/head.hbs file instead.\n\
                     The tracking code may be found in the Google Analytics Admin page.\n\
                   "
                );
            }
            if html_config.curly_quotes {
                warn!(
                    "The output.html.curly-quotes field has been renamed to \
                     output.html.smart-punctuation.\n\
                     Use the new name in book.toml to remove this warning."
                );
            }
        }

    }
}